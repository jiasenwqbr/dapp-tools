// SPDX-License-Identifier: MIT 
pragma solidity 0.8.23;
import "./SafeMath.sol";
contract Project {
    using SafeMath for uint256;
    // Data structures
    enum State {
        Fundraising,
        Expired,
        Successful
    }
    // State variables
    address payable  public creator;
    uint public amountGoal;
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
    constructor (
        address payable projectStarter,
        string memory projectTitle,
        string memory projectDesc,
        uint fundRaisingDeadline,
        uint goalAmount
    )  {
        creator = projectStarter;
        title = projectTitle;
        description = projectDesc;
        amountGoal = goalAmount;
        raiseBy = fundRaisingDeadline;
        currentBalance = 0;
    }
    /** @dev Function to fund a certain project.为某个项目提供资金的功能。
      */
    function contribute() external inState(State.Fundraising) payable {
        require(msg.sender != creator);
        contributions[msg.sender] = contributions[msg.sender].add(msg.value);
        currentBalance = currentBalance.add(msg.value);
        emit FundingReceived(msg.sender, msg.value, currentBalance);
        checkIfFundingCompleteOrExpired();
    }
    /** @dev Function to change the project state depending on conditions.
     * 根据条件更改项目状态的函数。
      */
    function checkIfFundingCompleteOrExpired() public {
        if (currentBalance >= amountGoal) {
            state = State.Successful;
        } else if (block.timestamp > raiseBy){
            state = State.Expired;
        } 
        completeAt = block.timestamp;
    }
     /** @dev Function to give the received funds to project starter.
      * 将收到的资金交给项目启动人的功能。
      */
    function payOut() internal inState(State.Successful) returns (bool){
        uint256 totalRaised = currentBalance;
        currentBalance = 0;
        if (creator.send(totalRaised)){
            emit CreatorPaid(creator);
            return true;
        } else {
            currentBalance = totalRaised;
            state = State.Successful;
        }
        return false;
    }
    /** @dev Function to retrieve donated amount when a project expires.
     * 用于在项目过期时获取捐赠金额。
      */

    function getRefund() public inState(State.Expired) returns (bool) {
        require(contributions[msg.sender] > 0);
        uint amountToRefund = contributions[msg.sender];
        address payable recipient = payable(msg.sender);
        if (recipient.send(amountToRefund)){
            contributions[msg.sender] = amountToRefund;
            return false;
        } else {
            currentBalance = currentBalance.sub(amountToRefund);
        }
        return true;
    }
    /** @dev Function to get specific information about the project.
      * Returns all the project's details
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