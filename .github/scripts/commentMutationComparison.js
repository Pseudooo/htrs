module.exports = ({github, context, mutantsOutput}) => {

    const masterSummary = getMasterSummary(mutantsOutput);
    const featureSummary = getFeatureSummary(mutantsOutput);

    const percentageDiff = featureSummary.percentageCaught - masterSummary.percentageCaught;

    let commentBody = "### Mutation Testing Completed\n";
    if(percentageDiff > 0) {
        commentBody += `:white_check_mark: Mutation Score Increased by +${percentageDiff.toFixed(2)}%\n`;
    } else if(percentageDiff < 0) {
        commentBody += `:warning: Mutation score decreased by ${percentageDiff.toFixed(2)}%\n`;
    } else {
        commentBody += `Mutation Score is unchanged\n`;
    }
    commentBody += '#### Summary\n'
    commentBody += '| Target | Total Mutants | Caught | Missed | Percentage Caught |\n';
    commentBody += '| ------ | ------------- | ------ | ------ | ----------------- |\n';
    commentBody += getSummaryTableRow('Master', masterSummary);
    commentBody += getSummaryTableRow('Branch', featureSummary);

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
    let percentageCaught = (caught / total) * 100;

    return {
        caught: caught,
        missed: missed,
        total: total,
        percentageCaught: percentageCaught,
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
        percentageCaught: percentageCaught,
    };
}

function getSummaryTableRow(branch, summary) {
    return `| ${branch} | ${summary.total} | ${summary.caught} | ${summary.missed} | ${summary.percentageCaught.toFixed(2)}% |\n`;
}