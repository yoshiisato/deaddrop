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
            pk_encryption=request.form['pk_encryption'],
            sc_addr=request.form['sc_addr'],
            constraints=request.form['constraints']
        )
        db.session.add(e)
        db.session.commit()
        flash("Entry saved!", "success")
        return redirect(url_for('show_list'))
    return render_template('register.html', current_year=datetime.utcnow().year)

@app.route('/submit_bug', methods=['GET', 'POST'])
def submit_bug():
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
        example_json=example_json,
        current_year=datetime.utcnow().year
    )

@app.route('/bug_lookup', methods=['GET', 'POST'])
def bug_lookup():
    bugs = None
    pk_detect = ""
    bug_id = ""
    found_bug = None

    if request.method == 'POST':
        form_type = request.form.get('form_type')

        if form_type == "detect_lookup":
            pk_detect = request.form.get('pk_detect', '').strip()

            if pk_detect == "*":
                # Show all bugs (debug mode)
                bugs = BugReport.query.order_by(BugReport.timestamp.desc()).all()
            else:
                # Simulated response from tool (replace with subprocess output later)
                bugs = []

        elif form_type == "bug_id_lookup":
            bug_id = request.form.get('bug_id', '').strip()
            if bug_id:
                found_bug = BugReport.query.filter_by(bugid=bug_id).first()

    return render_template("bug_lookup.html",
                           pk_detect=pk_detect,
                           bug_id=bug_id,
                           bugs=bugs,
                           found_bug=found_bug,
                           current_year=datetime.utcnow().year)


@app.route('/bug_reports')
def all_bug_reports():
    all_bugs = BugReport.query.order_by(BugReport.timestamp.desc()).all()
    return render_template("all_bug_reports.html", bugs=all_bugs)