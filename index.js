const {initDevice} = require('.');

const r = initDevice();

if(r){
    console.log('Device initialized');
} else {
    console.log('Device not initialized');
}