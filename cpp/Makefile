# Compiler and flags
CC = g++
CFLAGS = -Wall -Wextra -g

# Target executable
TARGET = $(N)

# Source files and corresponding object files
SRCS = $(TARGET).cpp
OBJS = $(SRCS:.cpp=.o)

# Default rule to build the target
all: $(TARGET)

# Linking the object files to create the executable
$(TARGET): $(OBJS)
	$(CC) $(CFLAGS) -o $@ $(OBJS)

# Rule to compile .c files into .o files
%.o: %.cpp
	$(CC) $(CFLAGS) -c $< -o $@

# Clean up build artifacts
clean:
	rm -f $(OBJS) $(TARGET)

# Rebuild from scratch
rebuild: clean all

