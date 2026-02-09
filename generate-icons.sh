#!/bin/bash
set -euo pipefail

python - <<'PY'
from pathlib import Path
from PIL import Image, ImageDraw, ImageFilter

out_dir = Path("src-tauri/icons")
out_dir.mkdir(parents=True, exist_ok=True)

def make_icon(size: int) -> Image.Image:
    img = Image.new("RGBA", (size, size), (0, 0, 0, 0))

    shadow = Image.new("RGBA", (size, size), (0, 0, 0, 0))
    sd = ImageDraw.Draw(shadow)
    pad = int(size * 0.09)
    radius = int(size * 0.23)
    sd.rounded_rectangle(
        (pad, pad + int(size * 0.02), size - pad, size - pad + int(size * 0.02)),
        radius=radius,
        fill=(0, 0, 0, 55),
    )
    shadow = shadow.filter(ImageFilter.GaussianBlur(radius=max(1, size // 40)))
    img.alpha_composite(shadow)

    base = Image.new("RGBA", (size, size), (0, 0, 0, 0))
    bd = ImageDraw.Draw(base)
    mask = Image.new("L", (size, size), 0)
    md = ImageDraw.Draw(mask)
    md.rounded_rectangle((pad, pad, size - pad, size - pad), radius=radius, fill=255)

    top = (248, 250, 252, 255)
    bottom = (220, 225, 234, 255)
    for y in range(size):
        t = y / (size - 1)
        r = int(top[0] * (1 - t) + bottom[0] * t)
        g = int(top[1] * (1 - t) + bottom[1] * t)
        b = int(top[2] * (1 - t) + bottom[2] * t)
        bd.line((0, y, size, y), fill=(r, g, b, 255))

    highlight = Image.new("RGBA", (size, size), (0, 0, 0, 0))
    hd = ImageDraw.Draw(highlight)
    hd.ellipse(
        (int(size * 0.16), int(size * -0.34), int(size * 0.84), int(size * 0.44)),
        fill=(255, 255, 255, 75),
    )
    base.alpha_composite(highlight)

    bd.rounded_rectangle(
        (pad, pad, size - pad, size - pad),
        radius=radius,
        outline=(186, 192, 201, 185),
        width=max(1, size // 80),
    )

    clipped = Image.new("RGBA", (size, size), (0, 0, 0, 0))
    clipped.paste(base, (0, 0), mask)
    img.alpha_composite(clipped)

    gd = ImageDraw.Draw(img)
    cx = cy = size // 2
    ring_r = int(size * 0.24)
    ring_w = max(2, size // 22)
    glyph = (48, 54, 63, 230)

    gd.ellipse(
        (cx - ring_r, cy - ring_r, cx + ring_r, cy + ring_r),
        outline=glyph,
        width=ring_w,
    )
    gd.line(
        (cx, cy, cx, cy - int(size * 0.11)),
        fill=glyph,
        width=max(2, size // 26),
    )
    gd.line(
        (cx, cy, cx + int(size * 0.09), cy + int(size * 0.03)),
        fill=glyph,
        width=max(2, size // 30),
    )

    crown_w = int(size * 0.11)
    crown_h = int(size * 0.04)
    gd.rounded_rectangle(
        (
            cx - crown_w // 2,
            cy - ring_r - int(size * 0.08),
            cx + crown_w // 2,
            cy - ring_r - int(size * 0.08) + crown_h,
        ),
        radius=max(1, size // 60),
        fill=(62, 68, 78, 220),
    )

    c = max(2, size // 40)
    gd.ellipse((cx - c, cy - c, cx + c, cy + c), fill=(62, 68, 78, 240))
    return img

master = make_icon(1024)
master.resize((32, 32), Image.Resampling.LANCZOS).save(out_dir / "32x32.png")
master.resize((128, 128), Image.Resampling.LANCZOS).save(out_dir / "128x128.png")
master.resize((256, 256), Image.Resampling.LANCZOS).save(out_dir / "128x128@2x.png")
master.resize((512, 512), Image.Resampling.LANCZOS).save(out_dir / "icon.png")
master.save(out_dir / "icon.icns", format="ICNS", sizes=[(16, 16), (32, 32), (64, 64), (128, 128), (256, 256), (512, 512), (1024, 1024)])
master.save(out_dir / "icon.ico", format="ICO", sizes=[(16, 16), (24, 24), (32, 32), (48, 48), (64, 64), (128, 128), (256, 256)])
print("Generated Apple-style icons in", out_dir)
PY
