#!/usr/bin/env python3
""                                                   

                                                           
                                                   
   
import json
from pathlib import Path

ROOT = Path(__file__).resolve().parent

# ---------------------------------------------------------------------------
                                                
#
                                                
                                                                 
                                                   
                                     
#
# Fixed recorded measurement files used by the paper.
# Summarization uses this mapping; it does not search alternative runs.
RESULT = "paper_arms_20260914.json"
RESULT_OVERRIDE = {
                                       
    "lodepng": "paper_arms_20260915_rules124.json",
                                                    
                                                             
                                                         
    "http-parser": "paper_arms_20260915_rules124.json",
    "lil": "paper_arms_20260915_rules124.json",
    "optipng-0.7.7": "paper_arms_20260915_rules124.json",
                                                   
    "fzy": "paper_arms_20260916_gatefix.json",
}


def result_name(proj: str) -> str:
    ""                
    return RESULT_OVERRIDE.get(proj, RESULT)


# project -> ordered [(paper-group label, [study.toml op names])]
# Ops not listed in any group land in "(other ops)" — still counted in union.
GROUPS = {
    "libopenaptx": [
        ("aptX encoding", ["encode", "encode_hd"]),
        ("aptX decoding", ["decode", "decode_hd"]),
        ("Stream recovery", ["decode_sync", "reset_stream"]),
    ],
    "http-parser": [
        ("Request parsing", ["parse_requests"]),
        ("Response parsing", ["parse_responses"]),
        ("Incremental parsing", ["parse_incremental"]),
        ("URL parsing", ["parse_url"]),
    ],
    "fzy": [
        ("Fuzzy scoring", ["match_batch", "match_positions_full"]),
        ("Choice-list search", ["choices_ops"]),
        ("Candidate-list parsing", ["choices_fread_op"]),
    ],
    "zopfli": [
        ("DEFLATE compression",
         ["compress_gzip", "compress_zlib", "compress_deflate", "options_matrix"]),
    ],
    "lil": [
        ("Script execution", ["run_script", "run_suite"]),
        ("Embedding API", ["api_surface"]),
    ],
    "miniz": [
        ("Compression", ["compress", "tdefl_matrix"]),
        ("Decompression", ["uncompress", "tinfl_lowlevel"]),
        ("Streaming", ["deflate_stream", "inflate_stream"]),
        ("Checksums", ["checksums"]),
        ("ZIP processing", ["zip_write", "zip_read", "zip_file"]),
    ],
    "lz4": [
        ("Compression", ["compress_fast", "compress_hc", "compress_hc_min",
                         "compress_hc_max", "compress_destsize",
                         "compress_destsize_hc", "compress_extstate"]),
        ("Decompression", ["decompress", "decompress_partial"]),
        ("Streaming", ["compress_stream", "compress_stream_hc",
                       "decompress_stream"]),
        ("Dictionary modes", ["compress_dict", "decompress_dict",
                              "compress_dict_hc", "decompress_partial_dict"]),
    ],
    "lodepng": [
        ("PNG decoding", ["decode32", "decode_keep", "decode_file"]),
        ("PNG encoding", ["encode32", "encode_meta", "encode_settings"]),
        ("Color conversion", ["convert"]),
        ("zlib processing", ["zlib_roundtrip"]),
        ("Chunk processing", ["chunk_crc"]),
    ],
    "libqrencode": [
        ("QR encoding", ["encode_matrix", "encode_8bit", "input_builder",
                         "split_op"]),
        ("Micro-QR encoding", ["encode_mqr"]),
        ("Structured append", ["structured"]),
    ],
    "optipng-0.7.7": [
        ("PNG optimization", ["optimize_o2"]),
        ("Format conversion", ["optimize_o1"]),
    ],
    "brotli": [
        ("Compression", ["compress", "compress_hq"]),
        ("Decompression", ["decompress"]),
        ("Streaming", ["stream_roundtrip"]),
        ("Dictionary modes", ["dict_roundtrip"]),
    ],
    "libxml2": [
        ("Parsing", ["parse_suite", "html_suite", "sax_suite", "reader_suite"]),
        ("Validation", ["valid_suite", "schema_suite", "relaxng_suite"]),
        ("XPath Evaluation", ["xpath_op"]),
        ("Serialization", ["writer_op", "c14n_op"]),
        ("Dictionary/URI/string utilities", ["dict_uri_str"]),
    ],
}


if __name__ == "__main__":
    # coverage report (needs op_coverage.json); import this module for GROUPS only.
    for proj, groups in GROUPS.items():
        f = ROOT / proj / "validation_workload" / "purebin" / "results" / "op_coverage.json"
        if not f.is_file():
            print(f"== {proj}: MISSING {f}")
            continue
        d = json.loads(f.read_text())
        denom = set(d["denominator_fns"])
        per_op = d["per_op"]
        grouped = set()
        print(f"\n== {proj}  (denominator {len(denom)} reachable owned fns)")
        union_groups = set()
        for label, ops in groups:
            s = set()
            missing = [o for o in ops if o not in per_op]
            for o in ops:
                if o in per_op:
                    s |= set(per_op[o]["fns"])
                    grouped.add(o)
            union_groups |= s
            tag = f"  !!missing ops: {missing}" if missing else ""
            print(f"  {label:<38} {len(s):>4}  {100*len(s)/len(denom):5.1f}%{tag}")
        other = [o for o in per_op if o not in grouped]
        s_other = set()
        for o in other:
            s_other |= set(per_op[o]["fns"])
        if other:
            print(f"  {'(other ops: ' + ', '.join(other) + ')':<38} "
                  f"{len(s_other):>4}  {100*len(s_other)/len(denom):5.1f}%")
        u_all = union_groups | s_other
        print(f"  {'UNION (paper groups)':<38} {len(union_groups):>4}  "
              f"{100*len(union_groups)/len(denom):5.1f}%")
        print(f"  {'UNION (all ops)':<38} {len(u_all):>4}  "
              f"{100*len(u_all)/len(denom):5.1f}%")
