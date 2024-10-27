main:
	cargo run

build:
	cargo build


test:
	python test.py

# Example make git m="commit message"	
git: 
	git add .
	git commit -m "$(m)"
	git push				