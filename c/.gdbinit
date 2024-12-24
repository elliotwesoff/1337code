tui enable
layout vert

define gdbload
python
import os
import gdb
gdb.execute(f'source gdb/{os.path.basename(gdb.current_progspace().filename)}.gdb')
end

