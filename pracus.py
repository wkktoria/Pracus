import requests
from bs4 import BeautifulSoup
import csv
from abc import ABC
import sys
from PySide6.QtCore import Qt, QSortFilterProxyModel, QAbstractTableModel
from PySide6.QtWidgets import (
    QApplication,
    QWidget,
    QTableView,
    QMainWindow,
    QVBoxLayout,
    QLineEdit,
)

CSV_FILENAME = "job_offers.csv"
all_job_offers = []


class JobOffer(ABC):
    def __init__(self, title, company, location, link):
        self._title = title
        self._company = company
        self._location = location
        self._link = link

    def get_row(self):
        return {
            "Title": self._title,
            "Company": self._company,
            "Location": self._location,
            "Link": self._link,
            "Source": "-",
        }

    def get_title(self):
        return self._title

    def get_company(self):
        return self._company

    def get_location(self):
        return self._location

    def get_link(self):
        return self._link

    def get_source(self):
        return "-"


class JustJoinItOffer(JobOffer):
    def get_row(self):
        return {
            "Title": self._title,
            "Company": self._company,
            "Location": self._location,
            "Link": self._link,
            "Source": "justjoin.it",
        }

    def get_source(self):
        return "justjoin.it"


class NoFluffJobsOffer(JobOffer):
    def get_row(self):
        return {
            "Title": self._title,
            "Company": self._company,
            "Location": self._location,
            "Link": self._link,
            "Source": "nofluffjobs.com",
        }

    def get_source(self):
        return "nofluffjobs.com"


class PracujOffer(JobOffer):
    def get_row(self):
        return {
            "Title": self._title,
            "Company": self._company,
            "Location": self._location,
            "Link": self._link,
            "Source": "pracuj.pl",
        }

    def get_source(self):
        return "pracuj.pl"


def get_offers_from_justjoinit():
    URI = "https://justjoin.it/job-offers/all-locations/java?experience-level=junior"
    page = requests.get(URI)
    soup = BeautifulSoup(page.content, "html.parser")
    offers = soup.find_all("div", attrs={"item": "[object Object]"})

    for offer in offers:
        title = offer.find("h3").text.strip()
        company_div = offer.find("div", class_="css-1mx97sn")
        company = company_div.find("span").text.strip()
        location = offer.find("span", class_="css-1o4wo1x").text.strip()
        link = "https://justjoin.it" + offer.find("a", href=True)["href"]

        all_job_offers.append(JustJoinItOffer(title, company, location, link))


def get_offers_from_nofluffjobs():
    URI = "https://nofluffjobs.com/pl/Java?criteria=seniority%3Dtrainee%2Cjunior"
    page = requests.get(URI)
    soup = BeautifulSoup(page.content, "html.parser")
    offers = soup.find_all("a", class_="posting-list-item")

    for offer in offers:
        title = offer.find("h3", class_="posting-title__position").text.strip()
        company = offer.find("h4", class_="company-name").text.strip()
        location = offer.find("span", class_="tw-text-right").text.strip()
        link = "https://nofluffjobs.com" + offer["href"]

        all_job_offers.append(NoFluffJobsOffer(title, company, location, link))


def get_offers_from_pracuj():
    URI = "https://it.pracuj.pl/praca?et=1%2C3%2C17&itth=38"
    page = requests.get(URI)
    soup = BeautifulSoup(page.content, "html.parser")
    offers = soup.find_all("div", attrs={"data-test": "default-offer"})

    for offer in offers:
        title = offer.find("h2", attrs={"data-test": "offer-title"}).text.strip()
        company = offer.find(
            "h3", attrs={"data-test": "text-company-name"}
        ).text.strip()
        location = offer.find("h4", attrs={"data-test": "text-region"}).text.strip()
        link = offer.find("a", href=True)["href"]

        all_job_offers.append(PracujOffer(title, company, location, link))


def create_csv_file():
    with open(CSV_FILENAME, "w+", newline="") as csv_file:
        fields = ["Title", "Company", "Location", "Link", "Source"]
        writer = csv.DictWriter(csv_file, fieldnames=fields)

        writer.writeheader()

        for offer in all_job_offers:
            writer.writerow(offer.get_row())


class TableModel(QAbstractTableModel):
    def __init__(self, data):
        super().__init__()
        self._columns = ["Title", "Company", "Location", "Link", "Source"]
        self._data = data

    def data(self, index, role):
        if role == Qt.DisplayRole:
            return self._data[index.row()][index.column()]

    def headerData(self, section: int, orientation: Qt.Orientation, role: int = ...):
        if orientation == Qt.Horizontal and role == Qt.DisplayRole:
            return self._columns[section]
        if orientation == Qt.Vertical and role == Qt.DisplayRole:
            return f"{section + 1}"

    def rowCount(self, index):
        return len(self._data)

    def columnCount(self, index):
        return len(self._data[0])


class MainWindow(QMainWindow):
    def __init__(self):
        super().__init__()
        self.setWindowTitle("Pracuś | Job Offers")
        self.resize(540, 480)

        self.table = QTableView()

        data = []

        for offer in all_job_offers:
            data.append(
                [
                    offer.get_title(),
                    offer.get_company(),
                    offer.get_location(),
                    offer.get_link(),
                    offer.get_source(),
                ]
            )

        self.model = TableModel(data)

        self.proxy_model = QSortFilterProxyModel()
        self.proxy_model.setFilterKeyColumn(-1)
        self.proxy_model.setSourceModel(self.model)

        self.table.setModel(self.proxy_model)

        self.searchbar = QLineEdit()
        self.searchbar.textChanged.connect(self.proxy_model.setFilterFixedString)

        layout = QVBoxLayout()
        layout.addWidget(self.searchbar)
        layout.addWidget(self.table)

        container = QWidget()
        container.setLayout(layout)

        self.setCentralWidget(container)


def run_gui():
    app = QApplication(sys.argv)

    window = MainWindow()
    window.show()

    app.exec()


def main():
    get_offers_from_justjoinit()
    get_offers_from_nofluffjobs()
    get_offers_from_pracuj()

    create_csv_file()

    run_gui()


if __name__ == "__main__":
    main()
