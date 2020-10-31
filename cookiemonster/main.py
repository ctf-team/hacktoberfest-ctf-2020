#!/usr/bin/env python3
from flask import Flask
from flask import render_template
from flask import make_response
from flask import request

app = Flask(__name__)

@app.route('/')
def start():
    id = request.cookies.get('userID')
    allowed = False

    if id is None:
        id = '0'
    elif id == '1':
        allowed = True

    resp = make_response(render_template('index.html', allowed=allowed))
    resp.set_cookie('userID', id)
    
    return resp


if __name__ == '__main__':
    app.run(host="0.0.0.0")