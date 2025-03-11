from PIL import Image
import numpy as np

# Load the BMP file
bmp_path = "/Users/dharmik_patel/Documents/Projects and Assignments/dangerousDave/scripts/assets/map.bmp"
image = Image.open(bmp_path)

# Get image dimensions
width, height = image.size
tile_size = 16  # Each tile is 16x16 pixels

# Number of levels and dimensions of each level
num_levels = 10
level_width_tiles = 100  # 100 tiles per level
level_height_tiles = 10   # 10 tiles per level
level_width_pixels = level_width_tiles * tile_size  # 1600 pixels
level_height_pixels = level_height_tiles * tile_size  # 160 pixels

# Ensure the image dimensions match expected size
assert width == level_width_pixels, "Unexpected image width"
assert height == num_levels * level_height_pixels, "Unexpected image height"

# Convert the image to grayscale for easier processing
image_gray = image.convert("L")  # Convert to grayscale (single channel)
image_data = np.array(image_gray)  # Convert to NumPy array

# Dictionary to store unique tiles and assign byte values
tile_map = {}
tile_counter = 0

# Function to extract and encode a tile
def get_tile_encoding(x, y):
    global tile_counter
    tile = image_data[y:y+tile_size, x:x+tile_size]
    tile_tuple = tuple(tile.flatten())  # Convert tile to a hashable tuple

    if tile_tuple not in tile_map:
        tile_map[tile_tuple] = tile_counter
        tile_counter += 1

    return tile_map[tile_tuple]

# Extract levels into encoded 2D arrays
encoded_levels = []
for level_idx in range(num_levels):
    level_start_y = level_idx * level_height_pixels  # Start position in the image

    level_array = np.zeros((level_height_tiles, level_width_tiles), dtype=np.uint8)

    for row in range(level_height_tiles):
        for col in range(level_width_tiles):
            x = col * tile_size
            y = level_start_y + (row * tile_size)
            level_array[row, col] = get_tile_encoding(x, y)

    encoded_levels.append(level_array)

# Display one level encoding as an example
import ace_tools as tools
import pandas as pd

df = pd.DataFrame(encoded_levels[0])
tools.display_dataframe_to_user(name="Encoded Level 1", dataframe=df)

# Save encoded levels as a numpy file for reuse
encoded_levels_path = "/results/encoded_levels.npy"
np.save(encoded_levels_path, encoded_levels)

encoded_levels_path
