run:
	./build/src/chocolate-doom

setup:
	mkdir build
	@cd build && cmake ../chocolate-doom && make
	cp DOOM.WAD build/src/DOOM.WAD

preinstall/arch:
	sudo pacman -S base-devel cmake sdl2 sdl2_mixer

preinstall/debian:
	sudo apt install build-essential cmake libsdl2-dev libsdl2-mixer-dev

clean:
	rm -rf build