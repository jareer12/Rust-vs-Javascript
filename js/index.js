const express = require("express");
const si = require("systeminformation");

const app = express();

app.get("/", async function (req, res) {
  const memory = await si.mem();
  res.json({
    free: memory.free,
    swap: memory.swap || 0,
    total: memory.total,
    used: memory.used,
  });
});

app.listen(3500, function () {
  console.log("Server started on http://127.0.0.1:3500");
});
