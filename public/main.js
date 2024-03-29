// write a json extractor
const jsonExtractor = response => response.json().then(data => data);

const fetchData = () => {
    const url = 'http://localhost:8080/';
    const options = {
        method: 'GET',
        headers: {
            'Content-Type': 'application/json'
        }
    };
    return fetch(url, options).then(jsonExtractor);
};

fetchData().then(data => console.log(data))