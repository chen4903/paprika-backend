def hsl_to_rgb(h, s, l):
    """Convert HSL color to RGB color"""
    def hue_to_rgb(p, q, t):
        if t < 0:
            t += 1
        if t > 1:
            t -= 1
        if t < 1 / 6:
            return p + (q - p) * 6 * t
        if t < 1 / 2:
            return q
        if t < 2 / 3:
            return p + (q - p) * (2 / 3 - t) * 6
        return p

    if s == 0:
        r = g = b = int(l * 255)
    else:
        q = l * (1 + s) if l < 0.5 else l + s - l * s
        p = 2 * l - q
        r = int(hue_to_rgb(p, q, h + 1 / 3) * 255)
        g = int(hue_to_rgb(p, q, h) * 255)
        b = int(hue_to_rgb(p, q, h - 1 / 3) * 255)
    return [r, g, b]

def generate_uniform_colors(num_colors):
    """Generate uniformly distributed and non repetitive RGB colors"""
    colors = set()
    while len(colors) < num_colors:
        hue = len(colors) / num_colors  # Uniform distribution of hue
        rgb = tuple(hsl_to_rgb(hue, 1.0, 0.5))  # Fixed saturation and brightness
        colors.add(rgb)
    return [list(color) for color in colors]

def format_as_rust_array(colors):
    """Format the color list as a Rust array"""
    rust_array = "const COLORS: [[u8; 3]; 145] = [\n"
    for color in colors:
        rust_array += f"    [{color[0]}, {color[1]}, {color[2]}],\n"
    rust_array = rust_array.rstrip(",\n") + "\n];"
    return rust_array

# Generate 145 evenly distributed and non repetitive RGB colors
num_colors = 145
colors = generate_uniform_colors(num_colors)

# Sort by RGB value from small to large
colors.sort()

# Format as a Rust array
rust_code = format_as_rust_array(colors)

# Result
print(rust_code)