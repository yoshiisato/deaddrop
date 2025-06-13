import json
from flask import Flask, render_template, request, redirect, url_for, flash
from models import db, Entry, BugReport
from datetime import datetime

app = Flask(__name__)
app.config.update({
    'SQLALCHEMY_DATABASE_URI': 'sqlite:///data.db',
    'SQLALCHEMY_TRACK_MODIFICATIONS': False,
    'SECRET_KEY': 'replace-with-a-secure-random-value',
})
db.init_app(app)

with app.app_context():
    db.create_all()

# Show list at the root URL
@app.route('/', methods=['GET'])
def show_list():
    entries = Entry.query.order_by(Entry.timestamp.desc()).all()
    return render_template('list.html', entries=entries, current_year=datetime.utcnow().year)

# New “register” page for adding entries
@app.route('/register', methods=['GET', 'POST'])
def register():
    if request.method == 'POST':
        e = Entry(
            pk_clue=request.form['pk_clue'],
            sc_addr=request.form['sc_addr'],
            constraints=request.form['constraints']
        )
        db.session.add(e)
        db.session.commit()
        flash("Entry saved!", "success")
        return redirect(url_for('show_list'))
    return render_template('register.html', current_year=datetime.utcnow().year)

@app.route('/submit_bug/<int:entry_id>', methods=['GET', 'POST'])
def submit_bug(entry_id):
    entry = Entry.query.get_or_404(entry_id)

    if request.method == 'POST':
        file = request.files.get('bugfile')
        if not file or not file.filename.lower().endswith('.json'):
            flash("Please upload a .json file", "danger")
        else:
            try:
                data = json.load(file)
            except ValueError:
                flash("Invalid JSON!", "danger")
            else:
                # ensure all required keys are present
                required = ['bugid', 'ciphertext', 'omr_payload', 'omr_clue', 'attestation']
                if not all(k in data for k in required):
                    flash("JSON must include keys: " + ", ".join(required), "danger")
                else:
                    bug = BugReport(
                        entry_id   = entry.id,
                        bugid      = data['bugid'],
                        ciphertext = data['ciphertext'],
                        omr_payload= json.dumps(data['omr_payload']),
                        omr_clue   = data['omr_clue'],
                        attestation= data['attestation'],
                    )
                    db.session.add(bug)
                    db.session.commit()
                    flash("Bug report submitted!", "success")
                    return redirect(url_for('show_list'))

    # example JSON to display
    example_json = {
      "bugid": "BR-1234",
      "ciphertext": "0xdeadbeef…",
      "omr_payload": {"foo": 1, "bar": 2},
      "omr_clue": "some-clue",
      "attestation": "signed-by-x509…"
    }

    return render_template(
        'submit_bug.html',
        entry=entry,
        example_json=example_json,
        current_year=datetime.utcnow().year
    )

@app.route('/bug_lookup', methods=['GET', 'POST'])
def bug_lookup():
    bugs = None
    pk_detect = ""

    if request.method == 'POST':
        pk_detect = request.form.get('pk_detect', '').strip()

        if pk_detect == "*":
            # show all bugs (debug mode)
            bugs = BugReport.query.order_by(BugReport.timestamp.desc()).all()
        else:
            # Placeholder: simulate tool loading data
            # Replace this with subprocess output or file read later
            bugs = [
                {
                    "bugid": "BR-1234",
                    "omr_clue": "some-clue",
                    "description": "0xdeadbeef…"
                },
                {
                    "bugid": "BR-5678",
                    "omr_clue": "other-clue",
                    "description": "0xbeefdead…"
                }
            ]

    return render_template("bug_lookup.html",
                           pk_detect=pk_detect,
                           bugs=bugs,
                           current_year=datetime.utcnow().year)
