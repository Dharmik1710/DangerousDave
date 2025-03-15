import os
import csv
from PIL import Image
import pandas as pd

def merge_tiles(image_folder, output_image="../assets/dangerous_dave_game_resources.png", output_csv="../assets/metadata/tiles_metadata.csv"):
    # Ensure output folder exists for PNG conversion
    converted_folder = os.path.join(image_folder, "converted")
    os.makedirs(converted_folder, exist_ok=True)
    
    # Convert images to PNG and store them in a new folder
    image_files = [f for f in os.listdir(image_folder) if f.lower().endswith(('.png', '.jpg', '.jpeg', '.bmp'))]
    converted_files = []
    
    for filename in image_files:
        img = Image.open(os.path.join(image_folder, filename)).convert("RGBA")  # Convert to PNG with transparency
        
        # Make black color (0,0,0) transparent
        img_data = img.getdata()
        new_data = [(r, g, b, 0) if (r, g, b) == (0, 0, 0) else (r, g, b, a) for r, g, b, a in img_data]
        img.putdata(new_data)
        
        new_filename = os.path.splitext(filename)[0] + ".png"
        new_path = os.path.join(converted_folder, new_filename)
        img.save(new_path, "PNG")
        converted_files.append(new_filename)
    
    # Load images from the converted folder
    images = [(f, Image.open(os.path.join(converted_folder, f)).convert("RGBA")) for f in converted_files]
    images.sort(key=lambda x: x[1].size[1], reverse=True)  # Sort by height (tallest first)
    
    metadata = []
    current_x, current_y = 0, 0
    max_row_height = 0
    max_width = max(img.size[0] for _, img in images)  # Ensure max width is calculated correctly
    packed_images = []
    
    for filename, img in images:
        width, height = img.size
        
        # If the image exceeds the max width, move to a new row
        if current_x + width > max_width:
            current_x = 0
            current_y += max_row_height  # Move down to new row
            max_row_height = 0  # Reset max row height
        
        metadata.append({
            "Tile Name": filename,
            "Offset X": current_x,
            "Offset Y": current_y,
            "Width": width,
            "Height": height
        })
        
        max_row_height = max(max_row_height, height)  # Update row height
        packed_images.append((img, current_x, current_y))
        current_x += width
    
    total_height = current_y + max_row_height
    merged_image = Image.new("RGBA", (max_width, total_height), (0, 0, 0, 0))  # Transparent background
    
    for img, x, y in packed_images:
        merged_image.paste(img, (x, y), img if img.mode == "RGBA" else None)
    
    merged_image.save(output_image, "PNG")
    print(f"\nMerged image saved as: {output_image}")
    
    df = pd.DataFrame(metadata)
    df.to_csv(output_csv, index=False)
    print(f"Tile metadata saved as: {output_csv}")

# Run the function with your tiles folder
tile_folder = "../assets/tiles"  # Change this to your folder path
merge_tiles(tile_folder)
