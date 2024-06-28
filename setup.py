import os
import getpass
import subprocess

def ask_db_name() -> str:
    return input("Enter your postgres db name: ")

def ask_user() -> str:
    is_current_user_owening_db = False
    while True:
        answer = str(input("Does your user owen db:(y/n) ")).lower()
        if answer not in ["y", "n"]:
            continue
        is_current_user_owening_db = answer == "y"
        break
    if is_current_user_owening_db:
        return os.getlogin()
    return str(input("Enter user owening this db: "))

def ask_db_password() -> str:
    return getpass.getpass("Enter password to db: ")

def run_diesel_migrations():
    print("Setup diesel")
    subprocess.run("diesel setup", shell=True, check=True)
    print("Running migrations")
    subprocess.run("diesel migration run", shell=True, check=True)

def ask_run_server():
    while True:
        answer = str(input("Do you want to run server now:(y/n) ")).lower()
        if answer not in ["y", "n"]:
            continue
        break
    if answer == "y":
        subprocess.run("cargo run --release", shell=True)

db_name = ask_db_name()
username = ask_db_name()
password = ask_db_password()
file = open(".env", mode="w")
file.write(f"DATABASE_URL=postgres://{username}:{password}@localhost/{db_name}\n 
           REDIS_URL=redis://127.0.0.1/")

run_diesel_migrations()
ask_run_server()
print("Setup Doen")