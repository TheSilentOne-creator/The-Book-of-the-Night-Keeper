from flask import Flask, request, jsonify

app = Flask(__name__)

CORRECT_PIN = "4247"

@app.route("/login", methods=["POST"])
def login():
    data = request.get_json()
    if data and data.get("pin") == CORRECT_PIN:
        return jsonify({"status": "ok", "message": "Login successful"}), 200
    return jsonify({"status": "error", "message": "Invalid PIN"}), 401

if __name__ == "__main__":
    app.run(host="127.0.0.1", port=3000, debug=True)