const fs = require('fs');

const fileData = fs.readFileSync('tmp', 'utf-8');
console.log(fileData);

const alphabet = ['A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L',
  'M', 'N', 'O', 'P', 'Q', 'R',  'S', 'T', 'U', 'V', 'W', 'X',
  'Y', 'Z'];
const a = fileData.split('\n').map(e => e.trim().split(' '));
console.log(a);

for (let i = 0; i < 26; i++) {
  for (let j = 0; j < 26; j++) {
    console.log('    m.insert("' + alphabet[i] + alphabet[j] + '".to_owned(), ' + a[i][j] + ');');
  }
}

