const express = require("express");
const bodyparser = require("body-parser");
const cors = require("cors");

const app = express();
app.use(bodyparser.json());
app.use(cors());

app.get("/", (req, res) => {
	res.send("You've reached the manczakapi.");
	console.log("-- served a / response");
});

app.get("/remoteip", (req, res) => {
	let ip = req.socket.remoteAddress;
	res.json(ip);
	console.log(`-- served a /remoteip response for ${ip}`);
});

app.listen(1313, () => {
	console.log(`listening on :1313`);
});
