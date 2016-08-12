var Agents = React.createClass({
    getInitialState: function() {
        return {
            agents: []
        }
    },

    componentDidMount: function() {
        PuppeteerApi.getAgents()
            .then(function(newAgents) {
                this.setState({
                    agents: newAgents
                });
            });
    },

    render: function() {
        return (
                <div class='agent__columns'>
                    <div class='agent__list'>
                        <div>
                            <h4>Agents:</h4>
                        </div>
                        <div id='agent-list'></div>
                        <button type='button' class='agent__create'>Create</button>
                    </div>

                    <div class='agent__details'>
                        <h4>Details:</h4>
                        <div id='agent-details'></div>
                    </div>
                </div>
            );
    }
});

var AgentCell = React.createClass({
    render: function() {
        return (
                <div class='agent__cell'>
                    <div>
                        <span id='agent-name'></span>
                        <span id='agent-active' class='active-flag'></span>
                    </div>
                </div>
                );
    }
});
