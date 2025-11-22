module.exports = ({github, context, mutantsOutput}) => {

    const featureTotalMutants = mutantsOutput.feature_caught + mutantsOutput.feature_missed;
    const featureCaughtPercentage = mutantsOutput.feature_caught / featureTotalMutants;

    github.rest.issues.createComment({
        issue_number: context.issue.number,
        owner: context.repo.owner,
        repo: context.repo.repo,
        body: `Caught ${featureCaughtPercentage.toFixed(2)}% of mutants! ${mutantsOutput.feature_caught}/${featureTotalMutants}`,
    });

}