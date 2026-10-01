/* Exhaustive lodepng driver — exercises decode/encode/inspect/chunk/CRC paths.
 *
 * Usage:
 *   lodepng_perf_driver <input.png>
 *
 * For the given input PNG, runs:
 *   1. lodepng_decode_file (decodes to memory)
 *   2. lodepng_decode_memory (slurp + decode)
 *   3. lodepng_inspect (header only)
 *   4. lodepng_decode with custom state (decoder settings)
 *   5. Re-encode with each {color type × bit depth × filter strategy × interlace}
 *      combination supported by lodepng_encode (skips invalid combinations).
 *   6. lodepng_encode_memory + lodepng_encode_file
 *   7. lodepng_chunk_* walk over the PNG bytes (length, type, ancillary,
 *      private, safetocopy, next, data_const, check_crc)
 *   8. lodepng_crc32 over a known buffer
 *   9. lodepng_palette_add / lodepng_color_mode_copy / lodepng_get_bpp /
 *      lodepng_get_channels / lodepng_is_*_type
 *   10. lodepng_convert between color modes
 */
#include "lodepng.h"
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

static unsigned char *slurp(const char *path, size_t *outsize) {
	FILE *f = fopen(path, "rb");
	if (!f) return NULL;
	fseek(f, 0, SEEK_END);
	long sz = ftell(f);
	if (sz <= 0) { fclose(f); return NULL; }
	fseek(f, 0, SEEK_SET);
	unsigned char *buf = (unsigned char *)malloc((size_t)sz);
	if (!buf) { fclose(f); return NULL; }
	size_t r = fread(buf, 1, (size_t)sz, f);
	fclose(f);
	if (r != (size_t)sz) { free(buf); return NULL; }
	*outsize = (size_t)sz;
	return buf;
}

/* Walk all chunks in a PNG buffer to exercise lodepng_chunk_* helpers. */
static void walk_chunks(const unsigned char *png, size_t pngsize) {
	if (pngsize < 8) return;
	const unsigned char *chunk = png + 8;
	const unsigned char *end = png + pngsize;
	while (chunk + 8 < end) {
		unsigned len = lodepng_chunk_length(chunk);
		char type[5];
		lodepng_chunk_type(type, chunk);
		(void)lodepng_chunk_ancillary(chunk);
		(void)lodepng_chunk_private(chunk);
		(void)lodepng_chunk_safetocopy(chunk);
		(void)lodepng_chunk_data_const(chunk);
		(void)lodepng_chunk_check_crc(chunk);
		if (type[0] == 'I' && type[1] == 'E' && type[2] == 'N' && type[3] == 'D') break;
		if (chunk + 12 + len > end) break;
		chunk = lodepng_chunk_next_const(chunk, end);
		if (!chunk || chunk >= end) break;
	}
}

static void try_encode(const unsigned char *image, unsigned w, unsigned h,
                       LodePNGColorType ct, unsigned bd, int filter, int interlace) {
	LodePNGState st;
	lodepng_state_init(&st);
	st.info_raw.colortype = LCT_RGBA;
	st.info_raw.bitdepth = 8;
	st.info_png.color.colortype = ct;
	st.info_png.color.bitdepth = bd;
	st.info_png.interlace_method = (unsigned)interlace;
	st.encoder.filter_strategy = (LodePNGFilterStrategy)filter;
	st.encoder.auto_convert = 1;
	st.encoder.add_id = 1;
	st.encoder.text_compression = 0;

	/* Optional text + iText + ICC + EXIF metadata to push lodepng_add_* paths. */
	lodepng_add_text(&st.info_png, "Comment", "exhaustive_driver");
	lodepng_add_itext(&st.info_png, "Title", "en", "Title-EN", "Title");
	unsigned char fake_icc[16];
	memset(fake_icc, 0xAA, sizeof(fake_icc));
	lodepng_set_icc(&st.info_png, "fake-icc", fake_icc, sizeof(fake_icc));
	unsigned char fake_exif[8] = {0,1,2,3,4,5,6,7};
	lodepng_set_exif(&st.info_png, fake_exif, sizeof(fake_exif));

	if (ct == LCT_PALETTE) {
		for (int i = 0; i < 256; ++i) {
			(void)lodepng_palette_add(&st.info_png.color,
			                          (unsigned char)i, (unsigned char)i,
			                          (unsigned char)i, 255);
			(void)lodepng_palette_add(&st.info_raw,
			                          (unsigned char)i, (unsigned char)i,
			                          (unsigned char)i, 255);
		}
	}

	unsigned char *out = NULL;
	size_t outsize = 0;
	unsigned err = lodepng_encode(&out, &outsize, image, w, h, &st);
	if (!err && out) walk_chunks(out, outsize);
	free(out);

	/* Clear stuff */
	lodepng_clear_text(&st.info_png);
	lodepng_clear_itext(&st.info_png);
	lodepng_clear_icc(&st.info_png);
	lodepng_clear_exif(&st.info_png);
	lodepng_state_cleanup(&st);
}

static void exercise_color_mode_helpers(void) {
	LodePNGColorMode a, b;
	lodepng_color_mode_init(&a);
	lodepng_color_mode_init(&b);
	a.colortype = LCT_RGBA;
	a.bitdepth = 8;
	(void)lodepng_color_mode_copy(&b, &a);
	(void)lodepng_get_bpp(&a);
	(void)lodepng_get_channels(&a);
	(void)lodepng_is_greyscale_type(&a);
	(void)lodepng_is_alpha_type(&a);
	(void)lodepng_is_palette_type(&a);
	(void)lodepng_has_palette_alpha(&a);
	(void)lodepng_can_have_alpha(&a);
	lodepng_palette_clear(&b);
	lodepng_color_mode_cleanup(&a);
	lodepng_color_mode_cleanup(&b);

	/* CRC over a known buffer */
	unsigned char buf[16];
	for (int i = 0; i < 16; ++i) buf[i] = (unsigned char)i;
	(void)lodepng_crc32(buf, sizeof(buf));

	/* error_text on a few codes */
	(void)lodepng_error_text(0);
	(void)lodepng_error_text(28);
	(void)lodepng_error_text(99);
}

static void exercise_convert(const unsigned char *image, unsigned w, unsigned h) {
	/* RGBA8 -> RGB8 */
	LodePNGColorMode src, dst;
	lodepng_color_mode_init(&src);
	lodepng_color_mode_init(&dst);
	src.colortype = LCT_RGBA;
	src.bitdepth = 8;
	dst.colortype = LCT_RGB;
	dst.bitdepth = 8;
	size_t dstbytes = (size_t)lodepng_get_raw_size(w, h, &dst);
	unsigned char *out = (unsigned char *)malloc(dstbytes);
	if (out) {
		(void)lodepng_convert(out, image, &dst, &src, w, h);
		free(out);
	}
	lodepng_color_mode_cleanup(&src);
	lodepng_color_mode_cleanup(&dst);

	/* color stats */
	LodePNGColorStats stats;
	lodepng_color_stats_init(&stats);
	LodePNGColorMode srgba;
	lodepng_color_mode_init(&srgba);
	srgba.colortype = LCT_RGBA;
	srgba.bitdepth = 8;
	(void)lodepng_compute_color_stats(&stats, image, w, h, &srgba);
	lodepng_color_mode_cleanup(&srgba);
}

int main(int argc, char *argv[]) {
	exercise_color_mode_helpers();
	if (argc < 2) {
		fprintf(stderr, "usage: %s <png>\n", argv[0]);
		return 1;
	}
	const char *path = argv[1];

	/* lodepng_decode32 (memory) + lodepng_decode24_file */
	{
		FILE *f = fopen(path, "rb");
		if (f) {
			fseek(f, 0, SEEK_END);
			long sz = ftell(f);
			fseek(f, 0, SEEK_SET);
			if (sz > 0 && sz <= 32*1024*1024) {
				unsigned char *buf = malloc((size_t)sz);
				if (buf && fread(buf, 1, sz, f) == (size_t)sz) {
					unsigned char *out = NULL;
					unsigned w, h;
					(void)lodepng_decode32(&out, &w, &h, buf, sz);
					free(out);
				}
				free(buf);
			}
			fclose(f);
		}
	}
	{
		unsigned char *out = NULL;
		unsigned w, h;
		(void)lodepng_decode24_file(&out, &w, &h, path);
		free(out);
	}

	/* 1) decode_file */
	unsigned char *image = NULL;
	unsigned w = 0, h = 0;
	unsigned err = lodepng_decode32_file(&image, &w, &h, path);
	if (err) {
		fprintf(stderr, "decode_file: %u %s\n", err, lodepng_error_text(err));
		return 2;
	}

	/* 2) decode_memory via slurp + decode_memory + decode24 */
	size_t insize = 0;
	unsigned char *png = slurp(path, &insize);
	if (png) {
		unsigned char *out24 = NULL;
		unsigned w24 = 0, h24 = 0;
		unsigned e24 = lodepng_decode24(&out24, &w24, &h24, png, insize);
		(void)e24;
		free(out24);

		unsigned char *outmem = NULL;
		unsigned wm = 0, hm = 0;
		unsigned em = lodepng_decode_memory(&outmem, &wm, &hm, png, insize,
		                                    LCT_RGBA, 8);
		(void)em;
		free(outmem);

		/* 3) inspect */
		LodePNGState ist;
		lodepng_state_init(&ist);
		unsigned wi = 0, hi = 0;
		(void)lodepng_inspect(&wi, &hi, &ist, png, insize);
		lodepng_state_cleanup(&ist);

		/* 4) decode with state */
		LodePNGState dst;
		lodepng_state_init(&dst);
		dst.decoder.color_convert = 1;
		dst.decoder.read_text_chunks = 1;
		dst.decoder.remember_unknown_chunks = 1;
		unsigned char *outst = NULL;
		unsigned ws = 0, hs = 0;
		(void)lodepng_decode(&outst, &ws, &hs, &dst, png, insize);
		free(outst);
		lodepng_state_cleanup(&dst);

		/* 7) walk chunks */
		walk_chunks(png, insize);

		/* chunk_find / chunk_find_const */
		const unsigned char *iend = lodepng_chunk_find_const(png, png + insize, "IEND");
		(void)iend;
		unsigned char *iend_mut = lodepng_chunk_find(png, png + insize, "IEND");
		(void)iend_mut;

		/* lodepng_state_copy */
		LodePNGState s1, s2;
		lodepng_state_init(&s1);
		lodepng_state_init(&s2);
		lodepng_state_copy(&s2, &s1);
		lodepng_state_cleanup(&s1);
		lodepng_state_cleanup(&s2);

		free(png);
	}

	/* lodepng_chunk_create (synthetic 4-byte data chunk) */
	{
		unsigned char *out = NULL;
		size_t outlen = 0;
		unsigned char data[4] = {1, 2, 3, 4};
		(void)lodepng_chunk_create(&out, &outlen, sizeof(data), "tEXt", data);
		free(out);
	}

	exercise_convert(image, w, h);

	/* 5) Re-encode across combinations.
	 * To keep runtime tractable, only encode if dimensions are small.
	 */
	if (w * h <= 64 * 64) {
		LodePNGColorType cts[] = { LCT_GREY, LCT_RGB, LCT_PALETTE, LCT_GREY_ALPHA, LCT_RGBA };
		int filters[] = { LFS_ZERO, LFS_MINSUM, LFS_ENTROPY, LFS_BRUTE_FORCE };
		for (size_t ci = 0; ci < sizeof(cts)/sizeof(cts[0]); ++ci) {
			LodePNGColorType ct = cts[ci];
			/* Pick a reasonable bit depth per color type. */
			unsigned bd_options[3] = {0, 0, 0};
			int n_bd = 0;
			switch (ct) {
				case LCT_GREY: bd_options[0] = 1; bd_options[1] = 8; n_bd = 2; break;
				case LCT_RGB: bd_options[0] = 8; n_bd = 1; break;
				case LCT_PALETTE: bd_options[0] = 8; n_bd = 1; break;
				case LCT_GREY_ALPHA: bd_options[0] = 8; n_bd = 1; break;
				case LCT_RGBA: bd_options[0] = 8; bd_options[1] = 16; n_bd = 2; break;
				default: n_bd = 0; break;
			}
			for (int bi = 0; bi < n_bd; ++bi) {
				for (size_t fi = 0; fi < sizeof(filters)/sizeof(filters[0]); ++fi) {
					try_encode(image, w, h, ct, bd_options[bi], filters[fi], 0);
				}
				/* interlace = 1 once per (ct, bd) */
				try_encode(image, w, h, ct, bd_options[bi], LFS_MINSUM, 1);
			}
		}
	} else {
		/* Large image: just one re-encode pass with default settings + file roundtrip */
		unsigned char *out = NULL;
		size_t outsize = 0;
		(void)lodepng_encode32(&out, &outsize, image, w, h);
		(void)lodepng_encode32_file("/tmp/_lodepng_cov.png", image, w, h);
		(void)lodepng_encode24_file("/tmp/_lodepng_cov24.png", image, w, h);
		free(out);
	}

	/* encode_memory (default RGBA8) */
	{
		unsigned char *out = NULL;
		size_t outsize = 0;
		(void)lodepng_encode_memory(&out, &outsize, image, w, h, LCT_RGBA, 8);
		free(out);
	}
	/* encode_file */
	(void)lodepng_encode_file("/tmp/_lodepng_cov_f.png", image, w, h, LCT_RGBA, 8);

	free(image);
	return 0;
}
