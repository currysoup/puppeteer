const Router = ReactRouter.Router;
const Route = ReactRouter.Route;
const IndexRoute = ReactRouter.IndexRoute;

class Header extends React.Component {
    constructor(props) {
        super(props);
    }

    render() {
        return (
            <header>
                <div class='container'>
                    <h3 class='header__title'>Puppeteer</h3>

                    <nav class='header__nav'>
                        <a class='header__nav-item' onclick='switchToAgents()'>Agents</a>
                        <a class='header__nav-item' onclick='switchToSensors()'>Sensors</a>
                    </nav>
                </div>
            </header>
        );
    }
}

class ReactApp extends React.Component {
    constructor(props) {
        super(props);
    }

    render() {
        return (
            <div className="wrapper">
                <div className="targeting-content">
                    <Header/>
                    {this.props.children}
                </div>
            </div>
        );
    }
}

class Agents extends React.Component {
    constructor() {
        super();
        this.state = {
            agents: []
        }
    }

    componentDidMount() {
        PuppeteerApi.getAgents()
            .then((newAgents) => {
                this.setState({
                    agents: newAgents
                });
            });
    }

    renderAgentCell(agent) {
        return (
                <div className='agent__cell'>
                    <div>
                        <span>{agent.name}</span>
                        {agent.active ? <span id='agent-active' className='active-flag'>✔️</span> : false }
                    </div>
                </div>
                );
    }

    render() {
        return (
                <div className='agent__columns'>
                    <div className='agent__list'>
                        <div>
                            <h4>Agents:</h4>
                        </div>
                        <div id='agent-list'>{this.state.agents.map(this.renderAgentCell, this)}</div>
                        <button type='button' className='agent__create'>Create</button>
                    </div>

                    <div className='agent__details'>
                        <h4>Details:</h4>
                        <div id='agent-details'></div>
                    </div>
                </div>
            );
    }
}


var AgentCell = React.createClass({
    render: function() {

    }
});

const routes = (
            <Route path='/' components={ReactApp}>
                <IndexRoute component={Agents}/>
            </Route>
        );

ReactDOM.render(<Router routes={routes} history={ReactRouter.browserHistory}/>, document.getElementById('react-mount'));
