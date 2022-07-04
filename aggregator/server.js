const path = require('path');
const express = require('express');
const cors = require('cors');

const app = express();

app.use(cors());
app.use(express.static('public'));

app.use((req, res, next) => {
  res.sendFile(path.join(__dirname, '../data', 'state.json'));
});

app.listen(9000, () => {
  console.log('Server started on port 9000');
});
