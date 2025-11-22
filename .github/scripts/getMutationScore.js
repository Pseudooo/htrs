const fs = require('node:fs');
const core = require('@actions/core');

module.exports = () => {
    const caughtCount = fs.readFileSync('mutants.out/caught.txt', 'utf8')
        .split('\n')
        .length - 1;
    const missedCount = fs.readFileSync('mutants.out/missed.txt', 'utf8')
        .split('\n')
        .length - 1;
    const unviableCount = fs.readFileSync('mutants.out/unviable.txt', 'utf8')
        .split('\n')
        .length - 1;

    core.setOutput('caught', caughtCount + unviableCount);
    core.setOutput('missed', missedCount);
}
