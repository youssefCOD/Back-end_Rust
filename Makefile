main:
	cargo run

build:
	cargo build

# Example make git m="commit message"	
git: 
	git add .
	git commit -m "$(m)"
	git push				