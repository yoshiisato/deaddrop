from flask_sqlalchemy import SQLAlchemy

db = SQLAlchemy()

class Entry(db.Model):
    id = db.Column(db.Integer, primary_key=True)
    pk_clue = db.Column(db.String(256), nullable=False)
    sc_addr = db.Column(db.String(256), nullable=False)
    constraints = db.Column(db.Text, nullable=False)
    timestamp = db.Column(db.DateTime, server_default=db.func.now())

from flask_sqlalchemy import SQLAlchemy

db = SQLAlchemy()

class Entry(db.Model):
    id = db.Column(db.Integer, primary_key=True)
    pk_clue = db.Column(db.String(256), nullable=False)
    sc_addr = db.Column(db.String(256), nullable=False)
    constraints = db.Column(db.Text, nullable=False)
    timestamp = db.Column(db.DateTime, server_default=db.func.now())

class BugReport(db.Model):
    id = db.Column(db.Integer, primary_key=True)
    entry_id    = db.Column(db.Integer, db.ForeignKey('entry.id'), nullable=False)
    bugid       = db.Column(db.String(128), nullable=False)
    ciphertext  = db.Column(db.Text, nullable=False)
    omr_payload = db.Column(db.Text, nullable=False)
    omr_clue    = db.Column(db.Text, nullable=False)
    attestation = db.Column(db.Text, nullable=False)
    timestamp   = db.Column(db.DateTime, server_default=db.func.now())

    entry = db.relationship('Entry', backref=db.backref('bugs', lazy=True))

