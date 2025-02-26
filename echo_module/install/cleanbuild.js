const fs = require("fs/promises");

async function cleanbuild() {
  await fs.rm("buildjs", { recursive: true, force: true });
}

cleanbuild();