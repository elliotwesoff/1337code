define gl
  python
import os
import gdb
gdb.execute(f'source gdb/{os.path.basename(gdb.current_progspace().filename)}.gdb')
  end
end


define ss
  if $argc == 0
    printf "Usage: ss <sleep>\n"
    printf "Ctrl+C to stop.\n"
    return
  end

  python
import time
while True:
    gdb.execute("step")
    gdb.execute("refresh", to_string=False)  # Force TUI refresh
    time.sleep(1.0)  # Adjust delay in seconds
  end
end

