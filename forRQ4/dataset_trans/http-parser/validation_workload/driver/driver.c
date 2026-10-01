/*
 * validation driver for http-parser — ONE driver, three link targets
 * (c / c2rust_raw / ours). Calls only the C ABI that c2rust preserves.
 *
 * CLI:
 *   driver <op> <input> <iters>
 *     parse_requests   : parse a pipelined request log (callbacks fold
 *                        methods/urls/headers/bodies; keep-alive queried)
 *     parse_responses  : parse a pipelined response log (incl. chunked)
 *     parse_url        : http_parser_parse_url over a URL list
 *     parse_incremental: 7-byte-chunk feeding + pause/unpause per message
 *     proto_utils      : method_str/status_str/errno tables, version,
 *                        set_max_header_size incl. overflow error path
 *   driver gen_http <ignored> <out_dir>
 *     writes requests.log, responses.log, urls.txt (realistic message mix
 *     modeled on http-parser's upstream test fixtures, ~2MB each)
 *
 * Output: ONE digest line on stdout (project convention).
 */
#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#include "http_parser.h"

/* 4-lane xor-rotate digest (project convention) */
static inline uint64_t rotl64(uint64_t x, int r) { return (x << r) | (x >> (64 - r)); }

static uint64_t digest64(uint64_t seed, const uint8_t *p, size_t n) {
    uint64_t l0 = seed ^ 0xcbf29ce484222325ULL;
    uint64_t l1 = 0x84222325cbf29ce4ULL;
    uint64_t l2 = 0x9ce484222325cbf2ULL;
    uint64_t l3 = 0x2325cbf29ce48422ULL;
    size_t i = 0;
    for (; i + 32 <= n; i += 32) {
        uint64_t w0, w1, w2, w3;
        memcpy(&w0, p + i,      8);
        memcpy(&w1, p + i + 8,  8);
        memcpy(&w2, p + i + 16, 8);
        memcpy(&w3, p + i + 24, 8);
        l0 = rotl64(l0 ^ w0, 17);
        l1 = rotl64(l1 ^ w1, 19);
        l2 = rotl64(l2 ^ w2, 23);
        l3 = rotl64(l3 ^ w3, 29);
    }
    for (; i < n; i++)
        l0 = rotl64(l0 ^ p[i], 11);
    uint64_t h = l0 * 0x9e3779b97f4a7c15ULL;
    h ^= rotl64(l1, 1); h *= 0x9e3779b97f4a7c15ULL;
    h ^= rotl64(l2, 2); h *= 0x9e3779b97f4a7c15ULL;
    h ^= rotl64(l3, 3); h *= 0x9e3779b97f4a7c15ULL;
    return h;
}

static uint8_t *read_file(const char *path, size_t *out_n) {
    FILE *f = fopen(path, "rb");
    if (!f) { fprintf(stderr, "cannot open %s\n", path); exit(2); }
    fseek(f, 0, SEEK_END);
    long n = ftell(f);
    fseek(f, 0, SEEK_SET);
    uint8_t *buf = malloc((size_t)n + 1);
    if (!buf || fread(buf, 1, (size_t)n, f) != (size_t)n) {
        fprintf(stderr, "cannot read %s\n", path); exit(2);
    }
    buf[n] = 0;
    fclose(f);
    *out_n = (size_t)n;
    return buf;
}

/* ------------------------------------------------------------------ */
/* parser callbacks folding everything into a per-run accumulator      */

struct acc {
    uint64_t h;
    uint64_t msgs, hdrs, body_bytes, chunks;
    int pause_armed;          /* parse_incremental: pause once per message */
    http_parser *parser;
};

static int cb_message_begin(http_parser *p) {
    struct acc *a = (struct acc *)p->data;
    a->msgs++;
    return 0;
}
static int cb_url(http_parser *p, const char *at, size_t n) {
    struct acc *a = (struct acc *)p->data;
    a->h = digest64(a->h, (const uint8_t *)at, n);
    return 0;
}
static int cb_status(http_parser *p, const char *at, size_t n) {
    struct acc *a = (struct acc *)p->data;
    a->h = digest64(a->h, (const uint8_t *)at, n);
    return 0;
}
static int cb_header_field(http_parser *p, const char *at, size_t n) {
    struct acc *a = (struct acc *)p->data;
    a->hdrs++;
    a->h = digest64(a->h, (const uint8_t *)at, n);
    return 0;
}
static int cb_header_value(http_parser *p, const char *at, size_t n) {
    struct acc *a = (struct acc *)p->data;
    a->h = digest64(a->h, (const uint8_t *)at, n);
    return 0;
}
static int cb_headers_complete(http_parser *p) {
    struct acc *a = (struct acc *)p->data;
    uint64_t meta[4] = {
        p->type == HTTP_REQUEST ? (uint64_t)p->method : (uint64_t)p->status_code,
        (uint64_t)p->http_major << 8 | p->http_minor,
        (uint64_t)http_should_keep_alive(p),
        (uint64_t)p->content_length,
    };
    a->h = digest64(a->h, (const uint8_t *)meta, sizeof(meta));
    return 0;
}
static int cb_body(http_parser *p, const char *at, size_t n) {
    struct acc *a = (struct acc *)p->data;
    a->body_bytes += n;
    a->h = digest64(a->h, (const uint8_t *)at, n);
    (void)http_body_is_final(p);
    return 0;
}
static int cb_message_complete(http_parser *p) {
    struct acc *a = (struct acc *)p->data;
    uint64_t fin[2] = { (uint64_t)http_should_keep_alive(p), 0xC0DA };
    a->h = digest64(a->h, (const uint8_t *)fin, sizeof(fin));
    if (a->pause_armed) http_parser_pause(a->parser, 1);
    return 0;
}
static int cb_chunk_header(http_parser *p) {
    struct acc *a = (struct acc *)p->data;
    a->chunks++;
    uint64_t cl = (uint64_t)p->content_length;
    a->h = digest64(a->h, (const uint8_t *)&cl, sizeof(cl));
    return 0;
}
static int cb_chunk_complete(http_parser *p) {
    struct acc *a = (struct acc *)p->data;
    a->h = digest64(a->h, (const uint8_t *)"ck", 2);
    return 0;
}

static void settings_fill(http_parser_settings *s) {
    http_parser_settings_init(s);
    s->on_message_begin = cb_message_begin;
    s->on_url = cb_url;
    s->on_status = cb_status;
    s->on_header_field = cb_header_field;
    s->on_header_value = cb_header_value;
    s->on_headers_complete = cb_headers_complete;
    s->on_body = cb_body;
    s->on_message_complete = cb_message_complete;
    s->on_chunk_header = cb_chunk_header;
    s->on_chunk_complete = cb_chunk_complete;
}

static void run_parse(const char *op, enum http_parser_type type,
                      const uint8_t *buf, size_t n, long iters) {
    http_parser_settings st;
    settings_fill(&st);
    uint64_t h = 0, msgs = 0;
    for (long i = 0; i < iters; i++) {
        struct acc a = {0};
        http_parser p;
        http_parser_init(&p, type);
        p.data = &a;
        a.parser = &p;
        size_t pos = 0;
        while (pos < n) {
            size_t np = http_parser_execute(&p, &st, (const char *)buf + pos, n - pos);
            pos += np;
            if (p.http_errno == HPE_OK) {
                if (pos >= n) break;
                if (p.upgrade) {
                    /* parser stops after Upgrade headers (remaining bytes
                     * belong to the upgraded protocol); next message = new
                     * parser, as a real server would do */
                    http_parser_init(&p, type);
                    p.data = &a;
                    continue;
                }
                fprintf(stderr, "%s stalled at %zu/%zu\n", op, pos, n);
                exit(3);
            }
            if (p.http_errno == HPE_CLOSED_CONNECTION) {
                /* Connection: close ended this "connection"; a real server
                 * would hand the next bytes to a fresh parser */
                http_parser_init(&p, type);
                p.data = &a;
                continue;
            }
            fprintf(stderr, "%s failed: %s at %zu/%zu\n", op,
                    http_errno_name((enum http_errno)p.http_errno), pos, n);
            exit(3);
        }
        http_parser_execute(&p, &st, NULL, 0);   /* EOF */
        h = digest64(h, (const uint8_t *)&a.h, sizeof(a.h));
        uint64_t cnts[3] = { a.msgs, a.hdrs, a.body_bytes };
        h = digest64(h, (const uint8_t *)cnts, sizeof(cnts));
        msgs = a.msgs;
    }
    printf("op=%s in=%zu out=%llu iters=%ld digest=%016llx\n",
           op, n, (unsigned long long)msgs, iters, (unsigned long long)h);
}

/* small-chunk feeding + pause/unpause across message boundaries */
static void run_parse_incremental(const uint8_t *buf, size_t n, long iters) {
    enum { CHUNK = 7 };
    http_parser_settings st;
    settings_fill(&st);
    uint64_t h = 0, msgs = 0;
    for (long i = 0; i < iters; i++) {
        struct acc a = {0};
        a.pause_armed = 1;
        http_parser p;
        http_parser_init(&p, HTTP_REQUEST);
        p.data = &a;
        a.parser = &p;
        size_t pos = 0;
        while (pos < n) {
            size_t c = (n - pos < CHUNK) ? (n - pos) : CHUNK;
            size_t np = http_parser_execute(&p, &st, (const char *)buf + pos, c);
            if (p.http_errno == HPE_PAUSED) {
                http_parser_pause(&p, 0);        /* resume */
                pos += np;
                continue;
            }
            if (p.http_errno != HPE_OK) {
                fprintf(stderr, "incremental failed: %s\n",
                        http_errno_name((enum http_errno)p.http_errno));
                exit(3);
            }
            pos += np;
        }
        h = digest64(h, (const uint8_t *)&a.h, sizeof(a.h));
        msgs = a.msgs;
    }
    printf("op=parse_incremental in=%zu out=%llu iters=%ld digest=%016llx\n",
           n, (unsigned long long)msgs, iters, (unsigned long long)h);
}

static void run_parse_url(const uint8_t *buf, size_t n, long iters) {
    uint64_t h = 0, urls = 0;
    for (long i = 0; i < iters; i++) {
        urls = 0;
        const char *p = (const char *)buf, *end = (const char *)buf + n;
        while (p < end) {
            const char *nl = memchr(p, '\n', (size_t)(end - p));
            if (!nl) nl = end;
            size_t len = (size_t)(nl - p);
            if (len > 0) {
                struct http_parser_url u;
                http_parser_url_init(&u);
                int r = http_parser_parse_url(p, len, 0, &u);
                uint64_t acc[3] = { (uint64_t)r, u.field_set, u.port };
                h = digest64(h, (const uint8_t *)acc, sizeof(acc));
                if (!r) {
                    for (int f = 0; f < UF_MAX; f++) {
                        if (u.field_set & (1 << f)) {
                            uint64_t fd[2] = { u.field_data[f].off, u.field_data[f].len };
                            h = digest64(h, (const uint8_t *)fd, sizeof(fd));
                        }
                    }
                }
                urls++;
            }
            p = nl + 1;
        }
    }
    printf("op=parse_url in=%zu out=%llu iters=%ld digest=%016llx\n",
           n, (unsigned long long)urls, iters, (unsigned long long)h);
}

static void run_proto_utils(long iters) {
    uint64_t h = 0;
    for (long i = 0; i < iters; i++) {
        uint64_t acc = 0;
        for (int m = 0; m <= 33; m++)
            acc += strlen(http_method_str((enum http_method)m));
        for (int s = 100; s <= 511; s++)
            acc += strlen(http_status_str((enum http_status)s));
        for (int e = 0; e <= 31; e++) {
            acc += strlen(http_errno_name((enum http_errno)e));
            acc += strlen(http_errno_description((enum http_errno)e));
        }
        acc += http_parser_version();
        h = digest64(h, (const uint8_t *)&acc, sizeof(acc));

        /* max-header-size clamp: an oversized header must trip the error */
        http_parser_set_max_header_size(64);
        http_parser_settings st;
        settings_fill(&st);
        struct acc a = {0};
        http_parser p;
        http_parser_init(&p, HTTP_REQUEST);
        p.data = &a;
        a.parser = &p;
        static const char big[] =
            "GET / HTTP/1.1\r\nX-Big: aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"
            "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa\r\n\r\n";
        http_parser_execute(&p, &st, big, sizeof(big) - 1);
        uint64_t err = (uint64_t)p.http_errno;
        h = digest64(h, (const uint8_t *)&err, sizeof(err));
        http_parser_set_max_header_size(0x7fffffff);   /* restore */
    }
    printf("op=proto_utils in=0 out=0 iters=%ld digest=%016llx\n",
           iters, (unsigned long long)h);
}

/* ------------------------------------------------------------------ */

static void emit(FILE *f, const char *s) { fputs(s, f); }

static void gen_http(const char *out_dir) {
    char path[1024];
    /* requests.log */
    snprintf(path, sizeof(path), "%s/requests.log", out_dir);
    FILE *f = fopen(path, "wb");
    if (!f) { fprintf(stderr, "cannot open %s\n", path); exit(2); }
    for (int r = 0; r < 1200; r++) {
        emit(f, "GET /index.html?q=search&page=2 HTTP/1.1\r\nHost: www.example.com\r\n"
                "User-Agent: Mozilla/5.0 (X11; Linux x86_64) Gecko/20100101 Firefox/115.0\r\n"
                "Accept: text/html,application/xhtml+xml,application/xml;q=0.9,*/*;q=0.8\r\n"
                "Accept-Language: en-US,en;q=0.5\r\nAccept-Encoding: gzip, deflate, br\r\n"
                "Cookie: session=abc123def456; theme=dark; tracking=0\r\n"
                "Connection: keep-alive\r\n\r\n");
        emit(f, "POST /api/v1/users HTTP/1.1\r\nHost: api.example.com\r\n"
                "Content-Type: application/json\r\nAuthorization: Bearer eyJhbGciOiJIUzI1NiJ9.x.y\r\n"
                "Content-Length: 55\r\n\r\n"
                "{\"name\":\"test user\",\"email\":\"t@example.com\",\"age\":30}\n\n");
        emit(f, "PUT /files/report.pdf HTTP/1.1\r\nHost: storage.example.com\r\n"
                "Content-Type: application/octet-stream\r\nContent-Length: 32\r\n"
                "Expect: 100-continue\r\n\r\n"
                "0123456789abcdef0123456789abcdef");
        emit(f, "DELETE /api/v1/users/42 HTTP/1.1\r\nHost: api.example.com\r\n"
                "Authorization: Bearer tok\r\n\r\n");
        emit(f, "HEAD /status HTTP/1.1\r\nHost: health.example.com\r\n\r\n");
        emit(f, "OPTIONS * HTTP/1.1\r\nHost: www.example.com\r\n\r\n");
        emit(f, "POST /upload HTTP/1.1\r\nHost: www.example.com\r\n"
                "Transfer-Encoding: chunked\r\nContent-Type: text/plain\r\n\r\n"
                "1a\r\nabcdefghijklmnopqrstuvwxyz\r\n10\r\n0123456789ABCDEF\r\n0\r\n\r\n");
        emit(f, "GET /chat HTTP/1.1\r\nHost: ws.example.com\r\nUpgrade: websocket\r\n"
                "Connection: Upgrade\r\nSec-WebSocket-Key: dGhlIHNhbXBsZSBub25jZQ==\r\n"
                "Sec-WebSocket-Version: 13\r\n\r\n");
    }
    fclose(f);
    /* responses.log */
    snprintf(path, sizeof(path), "%s/responses.log", out_dir);
    f = fopen(path, "wb");
    if (!f) { fprintf(stderr, "cannot open %s\n", path); exit(2); }
    for (int r = 0; r < 1500; r++) {
        emit(f, "HTTP/1.1 200 OK\r\nServer: nginx/1.24.0\r\nDate: Mon, 01 Jan 2024 00:00:00 GMT\r\n"
                "Content-Type: text/html; charset=utf-8\r\nContent-Length: 44\r\n"
                "Cache-Control: max-age=3600\r\nConnection: keep-alive\r\n\r\n"
                "<html><body><h1>Hello</h1></body></html>1234");
        emit(f, "HTTP/1.1 404 Not Found\r\nServer: Apache/2.4.57\r\nContent-Type: text/plain\r\n"
                "Content-Length: 14\r\n\r\npage not found");
        emit(f, "HTTP/1.1 301 Moved Permanently\r\nLocation: https://www.example.com/new\r\n"
                "Content-Length: 0\r\n\r\n");
        emit(f, "HTTP/1.1 200 OK\r\nTransfer-Encoding: chunked\r\nContent-Type: application/json\r\n\r\n"
                "b\r\n{\"ok\":true}\r\n9\r\n,{\"n\":42}\r\n0\r\n\r\n");
        emit(f, "HTTP/1.1 500 Internal Server Error\r\nContent-Length: 21\r\n"
                "Connection: close\r\n\r\ninternal server error");
        emit(f, "HTTP/1.1 204 No Content\r\nServer: nginx\r\n\r\n");
    }
    fclose(f);
    /* urls.txt */
    snprintf(path, sizeof(path), "%s/urls.txt", out_dir);
    f = fopen(path, "wb");
    if (!f) { fprintf(stderr, "cannot open %s\n", path); exit(2); }
    for (int r = 0; r < 4000; r++) {
        emit(f, "http://www.example.com/index.html\n");
        emit(f, "https://user:pass@api.example.com:8443/v1/users?limit=50&offset=100#results\n");
        emit(f, "/relative/path/to/resource?key=value\n");
        emit(f, "http://192.168.1.100:8080/admin\n");
        emit(f, "https://[2001:db8::1]:443/ipv6/path\n");
        emit(f, "ftp://files.example.com/pub/archive.tar.gz\n");
        emit(f, "http://example.com:65535/edge-port\n");
        emit(f, "https://sub.domain.example.co.uk/deep/nested/path/file.json?a=1&b=2&c=3\n");
    }
    fclose(f);
    fprintf(stderr, "wrote requests.log, responses.log, urls.txt under %s\n", out_dir);
}

int main(int argc, char **argv) {
    if (argc == 4 && strcmp(argv[1], "gen_http") == 0) {
        gen_http(argv[3]);
        return 0;
    }
    if (argc != 4) {
        fprintf(stderr, "usage: %s <op> <input> <iters> | %s gen_http x <out_dir>\n",
                argv[0], argv[0]);
        return 1;
    }
    const char *op = argv[1];
    long iters = strtol(argv[3], NULL, 10);
    if (iters <= 0) { fprintf(stderr, "bad iters\n"); return 1; }

    if (strcmp(op, "proto_utils") == 0) { run_proto_utils(iters); return 0; }

    size_t n;
    uint8_t *buf = read_file(argv[2], &n);
    if (strcmp(op, "parse_requests") == 0)        run_parse(op, HTTP_REQUEST, buf, n, iters);
    else if (strcmp(op, "parse_responses") == 0)  run_parse(op, HTTP_RESPONSE, buf, n, iters);
    else if (strcmp(op, "parse_incremental") == 0) run_parse_incremental(buf, n, iters);
    else if (strcmp(op, "parse_url") == 0)        run_parse_url(buf, n, iters);
    else { fprintf(stderr, "unknown op %s\n", op); return 1; }

    free(buf);
    return 0;
}
