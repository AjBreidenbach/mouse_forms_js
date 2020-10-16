var addon = require('../native');

//console.log(addon.compile('/home/andrew/projects/lawclick/forms/trademark-application_a.mf.pug'));

module.exports = function(file, obj) {
  let _obj = obj || {}
  let 
    compileResult = addon.compile(file, JSON.stringify(_obj)),
    output

  try {
    output = JSON.parse(compileResult)
  } catch(_) {
    output = compileResult
  }

  return output

}
