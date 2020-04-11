const express = require('express')
const bodyParser = require('body-parser')
const moment = require('moment')

const app = express()
const port = 8081

app.use(bodyParser.json())

app.use((req, res) => {
    console.log('[' + moment().format('hh:mm:ss') + '] '+ req.method + ':', req.path, req.body)
    res.send({code: 'OK'})
})

app.listen(port, () => console.log(`Example app listening at http://localhost:${port}`))