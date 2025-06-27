import type {Route} from "./+types/home";
import {Questions} from "~/components/questions";
import {useEffect, useState} from "react";


export function meta({}: Route.MetaArgs) {
    return [
        {title: "Meela Health"},
        {name: "description", content: "Meela Health"},
    ];
}
const sessionIdStorageItem = "sessionId";
const currentQuestionStorageItem = "currentQuestion";

export default function Home() {
    const [form, setForm] = useState({});
    const [sessionId, setSessionId] = useState("");
    const [numberOfQuestions, setNumberOfQuestions] = useState(0);
    const [currentQuestion, setCurrentQuestion] = useState(0);
    const [checkedAnswers, setCheckedAnswers] = useState(new Map<String, Set<String>>())


    useEffect(() => {
        fetch('http://localhost:3005/api/get-questions/')
            .then(r => r.json())
            .then(res => {
                setNumberOfQuestions(res.questions.length);
                setForm(res);
            });
    }, []);


    useEffect(() => {
        let existingSession = localStorage.getItem(sessionIdStorageItem);
        if (!existingSession) {
            fetch('http://localhost:3005/api/new-id/')
                .then(r => r.json())
                .then(res => {
                    localStorage.setItem(sessionIdStorageItem, res.id);
                    setSessionId(res.id);
                })
        } else {
            setSessionId(existingSession);
            fetch(`http://localhost:3005/api/get-answers/${existingSession}/`)
                .then(r => r.json())
                .then(res => {
                    let currentQuestionStorage = localStorage.getItem(currentQuestionStorageItem);
                    let currentQuestion = currentQuestionStorage == null ? 0 : parseInt(currentQuestionStorage, 10);
                    setCurrentQuestion(currentQuestion);
                    let parsedAnswers = new Map(
                        Object.entries(res).map(([key, arr]) => [key, new Set<String>(arr as string[])])
                    );
                    setCheckedAnswers(parsedAnswers)
                })
        }
    }, []);

    function nextQuestion(question: String, answers: Map<String, Set<String>>) {
        postAnswers(question, answers);
        if (currentQuestion < numberOfQuestions - 1) {
            let newCurrentQuestion = currentQuestion + 1;
            setCurrentQuestion(newCurrentQuestion);
            localStorage.setItem(currentQuestionStorageItem, String(newCurrentQuestion));
        }
    }

    function previousQuestion(question: String, answers: Map<String, Set<String>>) {
        postAnswers(question, answers);
        if (currentQuestion > 0) {
            let newCurrentQuestion = currentQuestion - 1;
            setCurrentQuestion(newCurrentQuestion);
            localStorage.setItem(currentQuestionStorageItem, String(newCurrentQuestion));
        }
    }

    function postAnswers(question: String, answers: Map<String, Set<String>>) {
        let checkedAnswers = answers.get(question);
        if (!checkedAnswers) {
            return;
        }

        fetch(`http://localhost:3005/api/post-answers/${sessionId}/${question}/`, {
            method: "POST",
            headers: {
                'Accept': 'application/json',
                'Content-Type': 'application/json'
            },

            body: JSON.stringify({
                answers: [...checkedAnswers],
            })
        });
    }

    return (
        <main className="flex items-center justify-center pt-16 pb-4">
            <div className="questions">
                <Questions data={form} answers={checkedAnswers} setAnswers={setCheckedAnswers}
                           questionIndex={currentQuestion}
                           nextQuestionAction={nextQuestion}
                           previousQuestionAction={previousQuestion}/>
            </div>
        </main>);
}




