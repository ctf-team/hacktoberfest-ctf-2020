#!/usr/bin/env python3
from flask import Flask
from flask import render_template
from flask import make_response
from flask import request

app = Flask(__name__)


@app.route('/', methods=['POST', 'GET'])
def start():
    if request.method == "GET":
        return render_template('index.html', title='Login')

    username = request.form['username']
    password = request.form['password']
    if username == 'admin' and password == '12345':
        return render_template('success.html')
    else:
        return render_template('index.html', title='Error', error='Incorrect username and/or password!')

if __name__ == '__main__':
    app.run(host="0.0.0.0")