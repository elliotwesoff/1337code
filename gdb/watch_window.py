import gdb
import curses

# Watched variable names
WATCHED_VARS = ["var1", "var2"]

# Function to fetch variable values
def get_variable_values():
    values = {}
    for var in WATCHED_VARS:
        try:
            value = gdb.parse_and_eval(var)
            values[var] = value
        except gdb.error:
            values[var] = "N/A"
    return values

# Curses UI to display variables
def display_variables(stdscr):
    stdscr.clear()
    while True:
        stdscr.clear()
        values = get_variable_values()
        stdscr.addstr(0, 0, "Watched Variables:")
        for idx, (var, val) in enumerate(values.items()):
            stdscr.addstr(idx + 1, 0, f"{var}: {val}")
        stdscr.refresh()
        # Wait a bit before updating
        stdscr.timeout(1000)

# Event to trigger display
class WatchWindow(gdb.Command):
    """Opens a custom window to display watched variables."""

    def __init__(self):
        super(WatchWindow, self).__init__("watch-window", gdb.COMMAND_USER)

    def invoke(self, arg, from_tty):
        curses.wrapper(display_variables)

WatchWindow()

