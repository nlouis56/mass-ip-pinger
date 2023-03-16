import mysql.connector
from dotenv import load_dotenv

class DatabaseConnector:
    def __init__(self, host=None, user=None, password=None, database=None):
        env = load_dotenv()
        if host is not None:
            self.host = host
        else:
            host = env.get('DB_HOST')
        if user is not None:
            self.user = user
        else:
            user = env.get('DB_USER')
        if password is not None:
            self.password = password
        else:
            password = env.get('DB_PASSWORD')
        if database is not None:
            self.database = database
        else:
            database = env.get('DB_DATABASE')

    def __del__(self):
        if self.connection.is_connected():
            self.disconnect()

    def connect(self):
        self.connection = mysql.connector.connect(
            host=self.host,
            user=self.user,
            password=self.password,
            database=self.database
        )
        return self.connection

    def disconnect(self):
        self.connection.close()

    def test_connection(self):
        if not self.connection:
            self.connect()
        if self.connection.is_connected():
            return True
        else:
            return False

    def insert_ip(self, ip, up=False, hostname=None):
        if not self.connection:
            self.connect()
        if self.connection.is_connected():
            cursor = self.connection.cursor()
            if up is False:
                up = 0
            else:
                up = 1
            if hostname is None:
                query = "INSERT INTO ip_list (ip, up) VALUES (%s, %s)"
                values = (ip, up)
            else:
                query = "INSERT INTO ip_list (ip, up, hostname) VALUES (%s, %s, %s)"
                values = (ip, up, hostname)
            cursor.execute(query, values)
            self.connection.commit()
            cursor.close()


if __name__ == "__main__":
    env = load_dotenv()
    host = env.get('DB_HOST')
    user = env.get('DB_USER')
    password = env.get('DB_PASSWORD')
    database = env.get('DB_DATABASE')
    dbc = DatabaseConnector(host=host, user=user, password=password, database=database)
    connection = dbc.connect()
    if connection.is_connected():
        print("Connected to database")
        dbc.disconnect()
    else:
        print("Connection failed")
