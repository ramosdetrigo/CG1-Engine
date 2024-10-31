# Makefile for SDL project

# Compiler
CXX = g++

# Compiler flags
CXXFLAGS = -O3 -march=native `sdl2-config --cflags`
# Ultimate optimization
# CXXFLAGS = -Ofast -march=native -flto -ftree-vectorize -finline-functions -g0 -ffunction-sections -fdata-sections -funroll-loops -ffast-math -Wl,--gc-sections -fno-strict-aliasing -funsafe-math-optimizations `sdl2-config --cflags`

# Linker flags
LDFLAGS = `sdl2-config --libs`

# Source files
SRCS = main.cpp

# Output executable
TARGET = cg1

# Default target
all: $(TARGET)

run: $(TARGET)
	./$(TARGET)

# Build the target
$(TARGET): $(SRCS)
	$(CXX) $(CXXFLAGS) -o $(TARGET) $(SRCS) $(LDFLAGS)

# Clean up build files
clean:
	rm -f $(TARGET)

# Phony targets
.PHONY: all clean