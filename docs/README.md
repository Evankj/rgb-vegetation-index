# RGB Vegetation Index Image Generator
This is a simple Rust program that takes an image and calculates a vegetation index (see below for currently supported VIs) for each pixel, then writes that pixel data to an output file for visualisation.

## Example
__Original Image:__
[original image](demo_img.jpeg)
__Green Minus Blue VI:__
[green minus blue image](gminusb.jpeg)

## Usage

The script takes 3 parameters:
1. `input` - Path to an input image.
2. `output` - Desired output image path.
3. `calculation` - The type of VI you would like calculated for the input image.

The VI `calculation` types currently supported are:
|Calculation Type|`calculation` value|
|---|---|
|Red Channel|`red`|
|Green Channel|`green`|
|Blue Channel|`blue`|
|Green - Blue Channel|`green-minus-blue`|
|Visible Atmospherically Resistant Index|`vari`|

To execute the script, simply run:

`cargo run <input> <output> --calculation <calculation>`


