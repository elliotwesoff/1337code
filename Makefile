# Compiler and flags
CC = gcc
CFLAGS = -Wall -Wextra -g

# Target executable
TARGET = 74

# Source files and corresponding object files
SRCS = $(TARGET).c
OBJS = $(SRCS:.c=.o)

# Default rule to build the target
all: $(TARGET)

# Linking the object files to create the executable
$(TARGET): $(OBJS)
	$(CC) $(CFLAGS) -o $@ $(OBJS)

# Rule to compile .c files into .o files
%.o: %.c
	$(CC) $(CFLAGS) -c $< -o $@

# Clean up build artifacts
clean:
	rm -f $(OBJS) $(TARGET)

# Rebuild from scratch
rebuild: clean all

