export function Questions({data, answers, setAnswers, questionIndex, previousQuestionAction, nextQuestionAction}: {
    data: any,
    answers: Map<String, Set<String>>,
    setAnswers: any,
    questionIndex: number,
    previousQuestionAction: (question: String, answers: Map<String, Set<String>>) => void,
    nextQuestionAction: (question: String, answers: Map<String, Set<String>>) => void
}) {

    function handleClick(checkbox: any, question: String, answer: String) {
        setAnswers((prevState: Map<String, Set<String>>) => {
            let newSet = new Set(prevState.get(question));
            if (checkbox.checked) {
                newSet.add(answer);
            } else {
                newSet.delete(answer);
            }
            let newMap = new Map(prevState);
            newMap.set(question, newSet);
            return newMap;
        })
    }

    if (!data.questions) {
        return (<div></div>)
    }

    return (
        <div>
            <p className="text-xl m-2">{data.questions && data.questions[questionIndex].pretty_name}</p>

            <form>
                {data.questions && data.questions[questionIndex].options.map((o: any) =>
                    <div><input key={o.name}
                                name={o.name}
                                value={o.name}
                                checked={!!answers.get(data.questions[questionIndex].name)?.has(o.name)}
                                onChange={(e) => handleClick(e.target, data.questions[questionIndex].name, o.name)}
                                type="checkbox"/><label htmlFor={o.name}>{o.pretty_name}</label></div>
                )}
            </form>
            <div className="questionButtons">
                <button
                    onClick={() => previousQuestionAction(data.questions[questionIndex].name, answers)}>Föregående
                </button>
                <button onClick={() => nextQuestionAction(data.questions[questionIndex].name, answers)}>Nästa
                </button>
            </div>
        </div>
    );
}
