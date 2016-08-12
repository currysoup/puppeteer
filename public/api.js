const PuppeteerApi = {
    getAgents: () => {
        return reqwest({
            url: '/api/agents',
            method: 'get',
            contentType: 'application/json'
        });
    }
}
