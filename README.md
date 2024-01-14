This is a project to get more familiar with Rust, egui, and the Google Sheets API. I used [this article](https://medium.com/@iamchrisgrounds/google-sheets-with-rust-6ecab23fa765) as the basis of the design, and have attempted to expand beyond its base functionality.

It involved:
- Getting the Google Sheets API set up, and getting a service account key.
- Creating a config file that can read the private_key.json file, and includes target sheet locations for functions.
- Setting up the authorization logic to enable access to the sheets
- Create rudimentary table in egui and display values from the sheet in the table

Currently it uses the Google Sheets API to interact with a sheet, and either a) display the values within a given range of the sheet, or b) return all values in the sheet, and display them as an egui table. 

Future plans include adding sheet editing functionality, improving the table quality,  more dynamic display of the sheet, and the ability to upload data from files to the next available rows (such as an inventory sheet).