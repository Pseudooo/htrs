module.exports = ({github, context, mutantsOutput}) => {

    const masterSummary = getMasterSummary(mutantsOutput);
    const featureSummary = getFeatureSummary(mutantsOutput);

    let commentBody = "### Mutation Testing Completed\n"
    commentBody += '| Target | Total Mutants | Caught | Missed | Percentage Caught |\n';
    commentBody += '| ------ | ------------- | ------ | ------ | ----------------- |\n';
    commentBody += `| Branch | ${featureSummary.total} | ${featureSummary.caught} | ${featureSummary.missed} | ${featureSummary.percentageCaught}% |\n`;

    github.rest.issues.createComment({
        issue_number: context.issue.number,
        owner: context.repo.owner,
        repo: context.repo.repo,
        body: commentBody,
    });
}

function getMasterSummary(output) {
    let caught = output.master_caught;
    let missed = output.master_missed;
    let total = caught + missed;
    let percentageCaught = (caught / total) * 100

    return {
        caught: caught,
        missed: missed,
        total: total,
        percentageCaught: percentageCaught.toFixed(2),
    };
}

function getFeatureSummary(output) {
    let caught = output.feature_caught;
    let missed = output.feature_missed;
    let total = caught + missed;
    let percentageCaught = (caught / total) * 100

    return {
        caught: caught,
        missed: missed,
        total: total,
        percentageCaught: percentageCaught.toFixed(2),
    };
}