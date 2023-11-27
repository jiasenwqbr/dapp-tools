# Creating a Simple Crowdfunding Dapp with Ethereum Solidity and Vue.js

A lot of us might have been in a situation wherein we had a brilliant idea, may it be of a product or an event, but just didn’t have the funds to bring it to life. This is where crowdfunding comes in.

我们中的很多人可能都遇到过这样的情况：我们有一个绝妙的想法，可能是一个产品或一个活动，但只是没有资金将其变为现实。 这就是众筹的用武之地。

You might have already heard of [Kickstarter](https://www.kickstarter.com/), or [CAMPFIRE](https://camp-fire.jp/), which are some of the known crowdfunding platforms that currently exist. In this tutorial, we’ll be building something similar, specifically a simple *decentralized* crowdfunding platform using Ethereum.

您可能已经听说过 [Kickstarter](https://www.kickstarter.com/) 或 [CAMPFIRE](https://camp-fire.jp/)，它们是目前存在的一些已知的众筹平台 。 在本教程中，我们将构建类似的东西，特别是使用以太坊的简单“去中心化”众筹平台。

## What We Are About To Build 我们要构建什么

In this tutorial, we’ll be building a simple crowdfunding dapp which will have the ‘minimum’ functionalities that you’d expect from a crowdfunding platform. Basically, we should be able to start crowdfunding projects, and fund them, which will look like this:

在本教程中，我们将构建一个简单的众筹 dapp，它将具有您期望从众筹平台获得的“最低”功能。 基本上，我们应该能够启动众筹项目并为其提供资金，如下所示：

![Crowdfunding](https://i.imgur.com/mmnPHLf.gif)

## Function List 功能列表

1. Start Project — we should be able to start a new crowdfunding project, along with setting its details like goal amount, etc.启动项目——我们应该能够启动一个新的众筹项目，并设置其详细信息，如目标金额等。
2. View Projects — we should be able to retrieve our existing projects’ details and display it 查看项目——我们应该能够检索现有项目的详细信息并显示它
3. Fund Project — we should be able to fund an existing project with Ether 资助项目——我们应该能够用以太币资助现有项目
4. Retrieve Funds — in the case where a project does not meet its goal amount, and is past its deadline, funders should be able to retrieve their contributed funds (All-or-Nothing setup) 收回资金——如果项目没有达到目标金额，并且已经超过截止日期，资助者应该能够收回他们贡献的资金（全有或全无设置）

## Tools

1. Smart Contract
   [Solidity](https://solidity.readthedocs.io/en/latest/), [Remix](https://remix.ethereum.org/), [Metamask](https://metamask.io/)
2. Frontend
   [Web3.js](https://web3js.readthedocs.io/en/1.0/), [Vue.js](https://vuejs.org/), [Vue-cli](https://cli.vuejs.org/), [Vuetify](https://vuetifyjs.com/en/)

Aside from that, we will also be using [Git](https://git-scm.com/) later to clone our boilerplate :)

## Prerequisites

* **Node** - v10.x.x (preferrably v10.15.0 for long term support)
* **npm** - v6.x.x (preferrably v6.6.0)

## Running It Locally

Clone this repo via to your local machine and install the dependencies by doing the following:

```bash
git clone https://github.com/openberry-ac/crowdfunding.git
cd crowdfunding
npm install
```

Start the application in a development environment via:

```bash
npm run serve
```

### Boilerplate

If it is only the boilerplate that you want to clone, do it via:

```
git clone https://github.com/openberry-ac/crowdfunding.git --branch boilerplate --single-branch
```

And then do the same following commands as shown above for starting the application, you should be all set by then.

## Built With

* [Solidity](https://solidity.readthedocs.io/en/v0.5.2/) - Ethereum's smart contract programming language
* [Vue.js](https://vuejs.org/) - The Javascript framework used
* [Vuetify](https://vuetifyjs.com/en/) - Material Design Component Framework
* [web3.js](https://github.com/ethereum/web3.js/) - Javascript library used to interact with the Ethereum blockchain 

## openberry

Anyone can become a blockchain engineer!

openberry is a tutorial marketplace, designed to allow anyone to learn blockchain programming.

Website: https://openberry.ac/

Medium: https://medium.com/openberry

Twitter: https://twitter.com/openberry_ac

# Why Make a Crowdfunding App?

Great ideas need effort and funding. You can ask for donations or solicitations, but donors would of course prefer giving to projects where they have some sort of certainty that the project is actually going somewhere.伟大的想法需要努力和资金。 你可以要求捐赠或募集，但捐赠者当然更愿意捐赠给那些他们确信该项目确实会取得进展的项目。

This is where crowdfunding comes in, an ideal setup where you can specify your goal, and a deadline for reaching it. If you miss your goal, the donations are returned, therefore reducing the risk for donors.这就是众筹的用武之地，这是一个理想的设置，您可以在其中指定您的目标以及实现目标的最后期限。 如果您未能实现目标，捐款将被退回，从而降低捐赠者的风险。

The existing platforms are all great, but in every completed project or in every donation sent, the platform takes a certain percent (margin) which could, at times, be too high for the project starters. What’s more, is that we heavily depend on them as the third-party that connects us to the donors. Should they fail, then we’re also in trouble.

现有的平台都很棒，但在每个完成的项目或发送的每笔捐款中，平台都会收取一定的百分比（利润），这有时对于项目启动者来说可能太高了。 更重要的是，我们严重依赖他们作为将我们与捐助者联系起来的第三方。 如果他们失败了，那么我们也会遇到麻烦。

With a decentralized setup, we can have a trustless platform and therefore the only fees everyone will pay are just the gas fees.

通过去中心化的设置，我们可以拥有一个无需信任的平台，因此每个人只需支付的费用只是gas费。

This is also one of the apps that are good to build early on, because you can definitely learn a lot by doing so.

这也是适合早期构建的应用程序之一，因为这样做绝对可以学到很多东西。



# Making the Project

## Workflow

1. Creating the Smart Contract
2. Building the Web App

# Creating the Smart Contract

We’ll be using [Solidity](https://solidity.readthedocs.io/en/v0.5.4/index.html), one of the programming languages used for creating smart contracts, specifically for Ethereum-based ones.

In [Remix](https://remix.ethereum.org/), create a new file named **Crowdfunding.sol** and add the following code:

```solidity
// We will be using Solidity version 0.5.4
pragma solidity 0.5.4;
// Importing OpenZeppelin's SafeMath Implementation
import 'https://github.com/OpenZeppelin/openzeppelin-solidity/contracts/math/SafeMath.sol';


contract Crowdfunding {
    using SafeMath for uint256;

    // List of existing projects
    Project[] private projects;

    // Event that will be emitted whenever a new project is started
    event ProjectStarted(
        address contractAddress,
        address projectStarter,
        string projectTitle,
        string projectDesc,
        uint256 deadline,
        uint256 goalAmount
    );

    /** @dev Function to start a new project.
      * @param title Title of the project to be created
      * @param description Brief description about the project
      * @param durationInDays Project deadline in days
      * @param amountToRaise Project goal in wei
      */
    function startProject(
        string calldata title,
        string calldata description,
        uint durationInDays,
        uint amountToRaise
    ) external {
        uint raiseUntil = now.add(durationInDays.mul(1 days));
        Project newProject = new Project(msg.sender, title, description, raiseUntil, amountToRaise);
        projects.push(newProject);
        emit ProjectStarted(
            address(newProject),
            msg.sender,
            title,
            description,
            raiseUntil,
            amountToRaise
        );
    }                                                                                                                                   

    /** @dev Function to get all projects' contract addresses.
      * @return A list of all projects' contract addreses
      */
    function returnAllProjects() external view returns(Project[] memory){
        return projects;
    }
}


contract Project {
    using SafeMath for uint256;
    
    // Data structures
    enum State {
        Fundraising,
        Expired,
        Successful
    }

    // State variables
    address payable public creator;
    uint public amountGoal; // required to reach at least this much, else everyone gets refund
    uint public completeAt;
    uint256 public currentBalance;
    uint public raiseBy;
    string public title;
    string public description;
    State public state = State.Fundraising; // initialize on create
    mapping (address => uint) public contributions;

    // Event that will be emitted whenever funding will be received
    event FundingReceived(address contributor, uint amount, uint currentTotal);
    // Event that will be emitted whenever the project starter has received the funds
    event CreatorPaid(address recipient);

    // Modifier to check current state
    modifier inState(State _state) {
        require(state == _state);
        _;
    }

    // Modifier to check if the function caller is the project creator
    modifier isCreator() {
        require(msg.sender == creator);
        _;
    }

    constructor
    (
        address payable projectStarter,
        string memory projectTitle,
        string memory projectDesc,
        uint fundRaisingDeadline,
        uint goalAmount
    ) public {
        creator = projectStarter;
        title = projectTitle;
        description = projectDesc;
        amountGoal = goalAmount;
        raiseBy = fundRaisingDeadline;
        currentBalance = 0;
    }

    /** @dev Function to fund a certain project.
      */
    function contribute() external inState(State.Fundraising) payable {
        require(msg.sender != creator);
        contributions[msg.sender] = contributions[msg.sender].add(msg.value);
        currentBalance = currentBalance.add(msg.value);
        emit FundingReceived(msg.sender, msg.value, currentBalance);
        checkIfFundingCompleteOrExpired();
    }

    /** @dev Function to change the project state depending on conditions.
      */
    function checkIfFundingCompleteOrExpired() public {
        if (currentBalance >= amountGoal) {
            state = State.Successful;
            payOut();
        } else if (now > raiseBy)  {
            state = State.Expired;
        }
        completeAt = now;
    }

    /** @dev Function to give the received funds to project starter.
      */
    function payOut() internal inState(State.Successful) returns (bool) {
        uint256 totalRaised = currentBalance;
        currentBalance = 0;

        if (creator.send(totalRaised)) {
            emit CreatorPaid(creator);
            return true;
        } else {
            currentBalance = totalRaised;
            state = State.Successful;
        }

        return false;
    }

    /** @dev Function to retrieve donated amount when a project expires.
      */
    function getRefund() public inState(State.Expired) returns (bool) {
        require(contributions[msg.sender] > 0);

        uint amountToRefund = contributions[msg.sender];
        contributions[msg.sender] = 0;

        if (!msg.sender.send(amountToRefund)) {
            contributions[msg.sender] = amountToRefund;
            return false;
        } else {
            currentBalance = currentBalance.sub(amountToRefund);
        }

        return true;
    }

    /** @dev Function to get specific information about the project.
      * @return Returns all the project's details
      */
    function getDetails() public view returns 
    (
        address payable projectStarter,
        string memory projectTitle,
        string memory projectDesc,
        uint256 deadline,
        State currentState,
        uint256 currentAmount,
        uint256 goalAmount
    ) {
        projectStarter = creator;
        projectTitle = title;
        projectDesc = description;
        deadline = raiseBy;
        currentState = state;
        currentAmount = currentBalance;
        goalAmount = amountGoal;
    }
}
```

We’re creating 2 contracts in one *.sol* file: *Crowdfunding* and *Project*. The **Crowdfunding** contract acts as a container for all Project contracts that will be initialized. Each crowdfunding project will have a contract of its own.

As for the **Project** contract, it is instantiated through the Crowdfunding contract, and handles all the methods that can be performed in every crowdfunding project, such as *contribute*(), *getRefund*(), etc.

Basically, each project will initially be in the *Fundraising* state, and will change states through the *checkIfFundingCompleteOrExpired*() function from there. Every time someone contributes funds towards the project, the state will change depending on certain conditions such has “has the project goal amount been met” or “has the project exceeded set deadline”.

Also, notice how we send funds from our contract in both the payOut() and getRefund() functions. We did not just directly send it, but had some security considerations because this is an area prone to a [re-entrancy attack](https://solidity.readthedocs.io/en/v0.5.3/security-considerations.html#re-entrancy).

And so, compile the **Crowdfunding** contract (make sure you select compiler version at the right side of Remix, and choose **0.5.4+commit.9549d8ff** because we are using Solidity version 0.5.4) and deploy it to the **Ropsten Test Network**. Make sure you are compiling and deploying the **Crowdfunding** contract.

To check that our contract was deployed, you should see the following:

![img](https://miro.medium.com/v2/resize:fit:996/1*aNaBsa7tSmyweepJUcrXbQ.png)

# Building the Web App

We’re done working on our smart contract. However, it doesn’t seem user-friendly and fun to just use Remix to interact with the contract, so we’ll be making a simple web application.

## Setting Up

To get up to speed, let’s clone a boilerplate project (found [here](https://github.com/openberry-ac/crowdfunding/tree/boilerplate)) by doing the following in a Terminal (or Command Prompt/Powershell for Windows):

```
# Cloning the boilerplate from GitHub
git clone -b boilerplate --single-branch https://github.com/openberry-ac/crowdfunding.git# Navigating to the directory and installing packages
cd crowdfunding
npm install# To run the app
npm run serve
```

In a few minutes, you should see the app running through a browser on [http://localhost:8080](http://localhost:8080/). Apparently, all we see is a white screen, because there is an error. If we open our console, we can see this:

![img](https://miro.medium.com/v2/resize:fit:1400/1*hMqbG04d_IyZk0etqGulbw.png)

Thing is, we have not connected our web app with our smart contract instance yet, which causes this error. And so, that is exactly what we’re about to do next.

## Connecting to Our Smart Contract Instance

To enable our web app to interact with our smart contract, we will be using [web3.js](https://github.com/ethereum/web3.js/). We already have the package installed, you can see how it is called in the file named **web3.js** inside the “contracts” folder, where we should put something like this:

<iframe src="https://medium.com/media/4cc5092574ad4718d9adcb78b1ec605b" allowfullscreen="" frameborder="0" height="479" width="680" title="web3.js" class="eh n ei vt bg" scrolling="no" style="box-sizing: inherit; top: 0px; width: 680px; left: 0px; height: 479px;"></iframe>

It basically loads the web3 instance the MetaMask extension initializes, which we will be needing later to interact with our smart contract.

You **might** encounter a MetaMask pop-up window that asks for access permission. This is because we have `ethereum.enable()` where the app requests for account (or wallet) access. You should just click the ‘Connect’ button right here:

![img](https://miro.medium.com/v2/resize:fit:752/1*NJoKFH25a6BtYuT5UvTNiw.png)

![img](https://miro.medium.com/v2/resize:fit:1400/1*lf2G6MW-q21TD6SR-4002Q.png)

Voila! You should see the app running through a browser on [http://localhost:8080](http://localhost:8080/) looking like this (though it is not functional yet):

![img](https://miro.medium.com/v2/resize:fit:1400/1*shaS7dL24HsV_qkV-XOMsA.png)

![img](https://miro.medium.com/v2/resize:fit:1400/1*_yhwlEFr9p7qRN9ugU3_Lg.png)

Now, we need our smart contract’s ABI to connect it to our web app. We have 2 contracts, so we need to get both their ABIs. To get it, go back to [Remix](https://remix.ethereum.org/), go to the **Compile** tab, and click **ABI** beside the Details button as shown in the picture (make sure **Crowdfunding** is selected):

![img](https://miro.medium.com/v2/resize:fit:978/1*wZxSyIH8vLuy2XXIuZpTig.png)

Copy the ABI by clicking the ABI button

After getting it, open the file named **crowdfundInstance.js** in the **contracts** folder, then paste it as the variable **abi** ’s value. There should be an example in the file, which you can always refer to.

Then, as you can see, we also need the deployed contract’s address, which you can get by going to [Remix](https://remix.ethereum.org/)’s Deploy tab, and clicking the copy icon on your deployed contract, as shown in this picture:

![img](https://miro.medium.com/v2/resize:fit:986/1*F2msplcObqmWUVnxZ7wtOA.png)

Copy the contract address by clicking the copy icon

Go back to **crowdfundInstance.js** in the **contracts** folder, then paste the address as the variable **address** ’ value. Save the file, and we’re done setting the contract information for our Crowdfunding contract. For the Project contract, just go back to [Remix](https://remix.ethereum.org/)’s Compile tab, select the **Project** contract, and get the ABI just like this:

![img](https://miro.medium.com/v2/resize:fit:996/1*GISUKdZvtXjyvtmqawBXqQ.png)

Copy the ABI by clicking the ABI button

After getting it, open the file named **crowdfundProjectInstance.js** in the **contracts** folder, then paste it as the variable **abi** ’s value. There should be an example in the file, which you can always refer to. We’re all set!

## Defining the Methods

As we see in the browser, our user interface is ready, but apparently, nothing is functional yet. That’s because we have not defined our functions yet, which we will be doing now. Go back to **App.vue**, and go to line 221 where you can see **methods**, but everything just contains a *console.log()*.

Our first function is retrieving all projects and their details. Let’s modify the *getProjects()* function to look like this:

<iframe src="https://medium.com/media/bea916bc306cb8ca35378eb7fe66bca5" allowfullscreen="" frameborder="0" height="325" width="680" title="App.vue" class="eh n ei vt bg" scrolling="no" style="box-sizing: inherit; top: 0px; width: 680px; left: 0px; height: 325px;"></iframe>

For this part, we simply call the *returnAllProjects*() function of our Crowdfunding contract to get all the projects’ contract addresses. Then, for each of them, we call *getDetails*() and store them in our defined objects, *projectInstances*, *projectData*, etc.

Next, we modify the function for starting a new project, *startProject()*:

<iframe src="https://medium.com/media/076fc6364ab17562fec07d2568bf0995" allowfullscreen="" frameborder="0" height="457" width="680" title="App.vue" class="eh n ei vt bg" scrolling="no" style="box-sizing: inherit; top: 0px; width: 680px; left: 0px; height: 456.992px;"></iframe>

Here, we’re just calling Crowdfunding contract’s *startProject*() function, while passing the data that has been inputted through the form. The contract then receives the data and creates a new project. After that, we get data being returned by the **ProjectStarted** event, and store the new project, along with some default values, in the web app too.

Note that we use `this.account` in this part, which was retrieved in the *mounted*() part of the code (executed when component has been *mounted*). It’s basically the current account that is active in Metamask.

![img](https://miro.medium.com/v2/resize:fit:1400/1*lf2G6MW-q21TD6SR-4002Q.png)

Now that we’ve set the functions for starting and viewing projects, let’s see them in action, shall we?

Let’s start a new project by clicking “Start a Project”, filling the form, and then submitting it. Note that there aren’t any form validation in place, so it’d be ideal if you input valid values for now :)

Once that’s done, your newly created project should immediately show up on the projects list like this:

![img](https://miro.medium.com/v2/resize:fit:1400/1*cjp8PKI1jkS2PCgEa1i3Ww.png)

New project now shows in project list. Easy!

Note: The app might not appear like the one shown on the image though. The interface for funding the project might not show because **project creators are not allowed to fund their own project**.

![img](https://miro.medium.com/v2/resize:fit:1400/1*_yhwlEFr9p7qRN9ugU3_Lg.png)

Moving on, we have the method that handles the event when a user funds a certain project, *fundProject()*:

<iframe src="https://medium.com/media/53dc9428ca585333ab5b98ad6e39e7fd" allowfullscreen="" frameborder="0" height="538" width="680" title="App.vue" class="eh n ei vt bg" scrolling="no" style="box-sizing: inherit; top: 0px; width: 680px; left: 0px; height: 538px;"></iframe>

Remember the contribute() function in our smart contract? We just call that function in *fundProject*() which sends the amount the user inputted in to the form.

![img](https://miro.medium.com/v2/resize:fit:1400/1*lf2G6MW-q21TD6SR-4002Q.png)

After defining the *fundProject*() function, we should now be able to use the previously ‘*not functional*’ fund button. What’s more is that immediately after the transaction is completed, the progress bar is updated. It should look like this:

![img](https://miro.medium.com/v2/resize:fit:1400/1*E58_oizyVX2-iCRM2J7bgw.gif)

Funding a project

![img](https://miro.medium.com/v2/resize:fit:1400/1*_yhwlEFr9p7qRN9ugU3_Lg.png)

Finally, we have *getRefund*(), which allows the user to retrieve his or her previously donated funds to an already expired (past deadline) project.

<iframe src="https://medium.com/media/573499dfaa3144af89ee328372bc1ef5" allowfullscreen="" frameborder="0" height="215" width="680" title="App.vue" class="eh n ei vt bg" scrolling="no" style="box-sizing: inherit; top: 0px; width: 680px; left: 0px; height: 215px;"></iframe>

And, we’re done!

![img](https://miro.medium.com/v2/resize:fit:1400/1*lf2G6MW-q21TD6SR-4002Q.png)

Refresh your browser to see the changes. This time, the whole web app is complete, and everything is functional! You should then be able to use it like this:

![img](https://miro.medium.com/v2/resize:fit:1400/1*3QW6sKpA5K4ioilHDDTl5g.gif)

Sample usage. You did it!

You can see the final result on this [GitHub repository](https://github.com/openberry-ac/crowdfunding/) (master branch) which you can always refer to :)

![img](https://miro.medium.com/v2/resize:fit:1400/1*_yhwlEFr9p7qRN9ugU3_Lg.png)

# Conclusion

We just finished making our simple crowdfunding platform! Awesome!

We learned how to instantiate a contract using another contract, and many more concepts related to Solidity through this application. We also learned how to set up our own project using Vue.js, and created a simple application.

So, what’s next?

Ethereum’s website has a [crowdsale tutorial](https://www.ethereum.org/crowdsale) which you might want to check out. It’s similar to what we just did, but uses a different approach (more into having a Decentralized Autonomous Organization). You might also want to write everything from scratch as you already have an idea on how the whole thing works.

![img](https://miro.medium.com/v2/1*sJzvVSLTXSwvzMg80rAtAA.png)

On a side note, you might want to check out openberry’s previous tutorial, [Ethereum Solidity + Vue.js Tutorial Simple Auction Dapp within 10 minutes](https://medium.com/openberry/ethereum-solidity-vue-js-tutorial-simple-auction-dapp-within-10-minutes-76ba48156b2).

’til the next tutorial!

![img](https://miro.medium.com/v2/resize:fit:1400/1*lAb5iihB2SGv9NyKTrazDg.png)

openberry is a tutorial marketplace, designed to allow anyone to learn blockchain programming.
