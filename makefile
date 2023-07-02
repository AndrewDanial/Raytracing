run:
	cargo run --release > image.ppm
	convert image.ppm image.png
	