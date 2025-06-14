from flask_sqlalchemy import SQLAlchemy

db = SQLAlchemy()

class Entry(db.Model):
    id = db.Column(db.Integer, primary_key=True)
    pk_encryption = db.Column(db.String(256), nullable=False)
    pk_clue = db.Column(db.String(256), nullable=False)
    sc_addr = db.Column(db.String(256), nullable=False)
    constraints = db.Column(db.Text, nullable=False)
    timestamp = db.Column(db.DateTime, server_default=db.func.now())

class BugReport(db.Model):
    id = db.Column(db.Integer, primary_key=True)
    bugid       = db.Column(db.String(128), nullable=False)
    ciphertext  = db.Column(db.Text, nullable=False)
    omr_payload = db.Column(db.Text, nullable=False)
    omr_clue    = db.Column(db.Text, nullable=False)
    attestation = db.Column(db.Text, nullable=False)
    timestamp   = db.Column(db.DateTime, server_default=db.func.now())
