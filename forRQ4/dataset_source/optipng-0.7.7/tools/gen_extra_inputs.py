#!/usr/bin/env python3
"""Generate extra synthetic inputs to push optipng's pngxtern/minitiff/
gifread/pnmio coverage past 80%.

Outputs into dataset_trans_process/optipng-0.7.7/workloads/inputs/experiment/.
"""
import struct, os, sys, pathlib

OUT = pathlib.Path(
    "/home/anonymous/artifact/PerfTrans/"
    "dataset_trans_process/optipng-0.7.7/workloads/inputs/experiment"
)
OUT.mkdir(parents=True, exist_ok=True)


def write(name: str, data: bytes) -> None:
    p = OUT / name
    p.write_bytes(data)
    print(f"wrote {p} ({len(data)} bytes)")


def big_endian_tiff() -> bytes:
    """4x2 RGB big-endian TIFF — forces get_ushort_m / get_ulong_m /
    read_ulong_values in src/minitiff/tiffread.c.
    """
    W, H = 4, 2
    # 4 strips of RGB888 — 1 row each so we get more StripOffsets/ByteCounts.
    row = bytes([255, 0, 0, 0, 255, 0, 0, 0, 255, 255, 255, 0]) * W // 12 if False else None
    pixel_row = b''.join(struct.pack('BBB', (i * 60) % 256, (i * 90) % 256, (i * 30) % 256) for i in range(W))
    image_data = pixel_row * H

    # IFD entries we'll write (tag, type, count, value-or-offset)
    # type 3 = SHORT (2), type 4 = LONG (4)
    entries = [
        (256, 3, 1, W),               # ImageWidth (SHORT)
        (257, 3, 1, H),               # ImageLength (SHORT)
        (258, 3, 3, 0),               # BitsPerSample [8,8,8] → offset filled later
        (259, 3, 1, 1),               # Compression = none
        (262, 3, 1, 2),               # PhotometricInterpretation = RGB
        (273, 4, 1, 0),               # StripOffsets → filled
        (277, 3, 1, 3),               # SamplesPerPixel = 3
        (278, 3, 1, H),               # RowsPerStrip
        (279, 4, 1, len(image_data)), # StripByteCounts
        (282, 5, 1, 0),               # XResolution (RATIONAL) → offset filled
        (283, 5, 1, 0),               # YResolution (RATIONAL) → offset filled
        (296, 3, 1, 2),               # ResolutionUnit = inch
    ]
    n_entries = len(entries)
    ifd_size = 2 + n_entries * 12 + 4

    # Layout (big-endian):
    #   header (8 bytes) | IFD | BitsPerSample [3*SHORT=6] | XRes [8] | YRes [8] | image_data
    header_size = 8
    ifd_offset = header_size
    bps_offset = ifd_offset + ifd_size
    xres_offset = bps_offset + 6
    yres_offset = xres_offset + 8
    strip_offset = yres_offset + 8

    # Patch tag values
    new_entries = []
    for tag, typ, cnt, val in entries:
        if tag == 258: val = bps_offset
        elif tag == 273: val = strip_offset
        elif tag == 282: val = xres_offset
        elif tag == 283: val = yres_offset
        new_entries.append((tag, typ, cnt, val))

    buf = bytearray()
    # Header: 'MM' (big-endian), magic 42, offset to first IFD
    buf += b'MM\x00\x2A'
    buf += struct.pack('>I', ifd_offset)
    # IFD
    buf += struct.pack('>H', n_entries)
    for tag, typ, cnt, val in new_entries:
        buf += struct.pack('>HHI', tag, typ, cnt)
        if typ == 3 and cnt == 1:
            # SHORT in upper 2 bytes of value field
            buf += struct.pack('>HH', val, 0)
        else:
            buf += struct.pack('>I', val)
    buf += struct.pack('>I', 0)  # next IFD = 0
    # BitsPerSample
    buf += struct.pack('>HHH', 8, 8, 8)
    # XResolution = 72/1
    buf += struct.pack('>II', 72, 1)
    # YResolution = 72/1
    buf += struct.pack('>II', 72, 1)
    # Strip data
    buf += image_data
    return bytes(buf)


def gif_with_extension() -> bytes:
    """1x1 GIF89a containing a Comment Extension and a Plain Text Extension —
    forces GIFSkipDataBlocks in src/gifread/gifread.c."""
    # Header
    buf = bytearray(b'GIF89a')
    # Logical Screen Descriptor: 1x1, GCT flag on, color resolution 0, sort 0, GCT size 0 (2 entries)
    buf += struct.pack('<HH', 1, 1)
    buf += bytes([0b10000000, 0, 0])
    # Global Color Table: 2 entries (black, white)
    buf += bytes([0, 0, 0,  255, 255, 255])
    # Comment Extension (0x21 0xFE)
    buf += bytes([0x21, 0xFE, 5]) + b'hello' + bytes([0])
    # Plain Text Extension (0x21 0x01)
    buf += bytes([0x21, 0x01, 12]) + bytes(12) + bytes([4]) + b'TEXT' + bytes([0])
    # Image Descriptor at (0,0) 1x1, no local color table
    buf += bytes([0x2C]) + struct.pack('<HHHH', 0, 0, 1, 1) + bytes([0])
    # Image data: LZW min code size 2, 1 block of 2 bytes
    buf += bytes([2, 2, 0x44, 0x01, 0])
    # Trailer
    buf += bytes([0x3B])
    return bytes(buf)


def corrupt_gif() -> bytes:
    """Truncated GIF — header + LSD then EOF. Forces ErrorRead/DefaultError/Alloc paths."""
    return b'GIF89a' + struct.pack('<HH', 64, 64) + bytes([0b10000000, 0, 0])


def indexed_bmp_1bit() -> bytes:
    """4x2 1-bit BMP — exercises bmp_memset_halfbytes / bmp_fread_halfbytes."""
    W, H = 4, 2
    # 1-bit BMP row padded to 4 bytes
    pixel_array = bytes([0b10100000, 0, 0, 0,
                         0b01010000, 0, 0, 0])
    palette = bytes([0, 0, 0, 0,  255, 255, 255, 0])
    info_header = struct.pack('<IIIHHIIIIII',
        40, W, H, 1, 1, 0, len(pixel_array), 2835, 2835, 2, 2)
    file_header = struct.pack('<HIHHI',
        0x4D42, 14 + len(info_header) + len(palette) + len(pixel_array),
        0, 0, 14 + len(info_header) + len(palette))
    return file_header + info_header + palette + pixel_array


def truncated_tiff() -> bytes:
    """TIFF header only — forces minitiff_error / default_error_handler."""
    return b'II\x2A\x00\x08\x00\x00\x00\x00\x00'


def fake_jpeg() -> bytes:
    """Minimal JFIF SOI/APP0 header — ≥12 bytes so pngxread.c dispatches
    to pngx_read_jpeg (which is stubbed to png_error)."""
    # SOI (FFD8) + APP0 marker (FFE0) + APP0 length (10) + "JFIF\0" + version + units + density + thumb
    return bytes([0xFF, 0xD8, 0xFF, 0xE0, 0x00, 0x10,
                  ord('J'), ord('F'), ord('I'), ord('F'), 0x00,
                  0x01, 0x01, 0x00, 0x00, 0x01, 0x00, 0x01, 0x00, 0x00,
                  0xFF, 0xD9])


def bmp_24bit() -> bytes:
    """4x2 24-bit RGB BMP (BI_RGB, no palette) — exercises bmp_memset_bytes
    inside pngxrbmp.c."""
    W, H = 4, 2
    row_size = (W * 3 + 3) & ~3
    pad = row_size - W * 3
    pixel_array = b''
    for y in range(H):
        for x in range(W):
            pixel_array += bytes([(x * 60) & 0xFF, (y * 100) & 0xFF, 200])
        pixel_array += bytes(pad)
    info_header = struct.pack('<IIIHHIIIIII',
        40, W, H, 1, 24, 0, len(pixel_array), 2835, 2835, 0, 0)
    file_header = struct.pack('<HIHHI',
        0x4D42, 14 + len(info_header) + len(pixel_array),
        0, 0, 14 + len(info_header))
    return file_header + info_header + pixel_array


def bmp_rle4() -> bytes:
    """8x4 4-bit RLE-compressed BMP (BI_RLE4 = 2) — exercises
    bmp_memset_halfbytes and bmp_fread_halfbytes."""
    W, H = 8, 4
    palette = b''
    for i in range(16):
        palette += bytes([i * 16, i * 16, i * 16, 0])
    rle_stream = b''
    for _ in range(H):
        rle_stream += bytes([8, 0x55])      # 8 pixels of idx 5 alternating
        rle_stream += bytes([0, 0])         # end of line
    rle_stream += bytes([0, 1])             # end of bitmap
    info_header = struct.pack('<IIIHHIIIIII',
        40, W, H, 1, 4, 2,
        len(rle_stream), 2835, 2835, 16, 16)
    file_header = struct.pack('<HIHHI',
        0x4D42, 14 + len(info_header) + len(palette) + len(rle_stream),
        0, 0, 14 + len(info_header) + len(palette))
    return file_header + info_header + palette + rle_stream


def pnm_p1_ascii() -> bytes:
    """ASCII PBM (P1) — exercises pnmin.c read_pbm_ascii + pnm_raw_sample_size."""
    return b'P1\n# ASCII PBM\n4 2\n1 0 1 0\n0 1 0 1\n'


def pnm_p2_ascii() -> bytes:
    """ASCII PGM (P2) — exercises pnmin.c read_pgm_ascii."""
    return b'P2\n# ASCII PGM\n4 2\n255\n10 100 200 250\n5 50 150 245\n'


def pnm_p3_ascii() -> bytes:
    """ASCII PPM (P3) — exercises pnmin.c read_ppm_ascii."""
    return b'P3\n# ASCII PPM\n4 2\n255\n255 0 0  0 255 0  0 0 255  128 128 0\n100 200 50  60 30 220  240 240 240  10 10 10\n'


def main():
    write('be_4x2.tif', big_endian_tiff())
    write('with_ext.gif', gif_with_extension())
    write('corrupt.gif', corrupt_gif())
    write('indexed_1bit.bmp', indexed_bmp_1bit())
    write('truncated.tif', truncated_tiff())
    write('fake.jpg', fake_jpeg())
    write('rgb24.bmp', bmp_24bit())
    write('rle4.bmp', bmp_rle4())
    write('ascii.pbm', pnm_p1_ascii())
    write('ascii.pgm', pnm_p2_ascii())
    write('ascii.ppm', pnm_p3_ascii())


if __name__ == '__main__':
    main()
