module.exports = ({github, context, mutantsOutput}) => {

    const featureTotalMutants = mutantsOutput.caught + mutantsOutput.missed;
    const featureCaughtPercentage = mutantsOutput.caught / featureTotalMutants;

    github.rest.issues.createComment({
        issue_number: context.issue.number,
        owner: context.repo.owner,
        repo: context.repo.repo,
        body: `Caught ${featureCaughtPercentage.toFixed(2)}% of mutants! ${mutantsOutput.caught}/${featureTotalMutants}`,
    });

}