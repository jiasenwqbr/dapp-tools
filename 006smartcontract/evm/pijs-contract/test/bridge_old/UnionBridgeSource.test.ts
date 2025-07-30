import { expect } from "chai";
import { ethers, upgrades } from "hardhat";
import { network } from "hardhat";
import { UnionBridgeSource } from  "../../typechain-types";

describe("UnionBridgeSource", () => {
    let unionBridgeSourceContract:UnionBridgeSource;
    let owner: any;
    let user1: any;
    let user2: any;
    let user3: any;
    let token:any;
    let feeDenominator = 10000;
    let feePercent = 500;

    beforeEach(async () => {
        [owner, user1 ,user2, user3] = await ethers.getSigners();
        const unionBridgeSourceFactory =  await ethers.getContractFactory("UnionBridgeSource");
        unionBridgeSourceContract = (await upgrades.deployProxy(unionBridgeSourceFactory,[
            owner.address,
            user2.address,
            owner.address,
            owner.address,
            feePercent],{
            initializer: "initialize",
        })) as UnionBridgeSource;
        await unionBridgeSourceContract.deployed();


        const TestERC20 = await ethers.getContractFactory("TestERC20");
        token = await TestERC20.deploy(owner.address); // 初始全部转给 deployer
        await token.deployed();
        console.log("TestERC20 deployed at:", token.address);

        const deployerBalance = await token.balanceOf(owner.address);
        console.log("Deployer token balance:", ethers.utils.formatEther(deployerBalance));

        // send 100 eth to user1
        const amount = ethers.utils.parseEther("100");
        const tx = await token.transfer(user1.address,amount);
        await tx.wait();

        const deployerBalance1 = await token.balanceOf(owner.address);
        console.log("Deployer token balance:", ethers.utils.formatEther(deployerBalance1));

        const user1Balance =  await token.balanceOf(user1.address);
        console.log("user1 balance:", ethers.utils.formatEther(user1Balance));

    });

    describe("Initialization",() => {
        it("shold owner has the role of manage",async () => {
            const  hasManageRole =  await unionBridgeSourceContract.hasRole(
                await unionBridgeSourceContract.MANAGE_ROLE(),
                owner.address
            );
            expect(hasManageRole).to.be.true;
        });
    });

    describe("depositeUNI",() => {
        it("depost UNI value shoud be correct", async () => {
            const params = {
                userAddr:owner.address,
                receiver:unionBridgeSourceContract.address,
                amount:ethers.utils.parseEther("1"),
                orderId:1,
                chainId:(await ethers.provider.getNetwork()).chainId
            };

            const types = {
                Permit: [
                    { name: "userAddr", type: "address" },
                    { name: "receiver", type: "address" },
                    { name: "amount", type: "uint256" },
                    { name: "orderId", type: "uint256" },
                    { name: "chainId", type: "uint256" },
                ],
            };
            const domain = {
                name: "UnionBridgeSource",
                version: "1",
                chainId: (await ethers.provider.getNetwork()).chainId,
                verifyingContract: unionBridgeSourceContract.address,
            };
            const signature = await owner._signTypedData(domain, types, params);
            const data = ethers.utils.defaultAbiCoder.encode(
                [
                   "address",
                   "address",
                   "uint256",
                   "uint256", 
                   "uint256",
                   "bytes"
                ],[
                    params.userAddr,
                    params.receiver,
                    params.amount,
                    params.orderId,
                    params.chainId,
                    signature
                ]
            );
            const ownerBeforeDepositeUNIBalance = await ethers.provider.getBalance(owner.address);
            await unionBridgeSourceContract.connect(owner).depositeUNI(data,{
                value: ethers.utils.parseEther("1"), 
                gasLimit: 1_000_000,
            });
           

            expect(await unionBridgeSourceContract.balance(ethers.constants.AddressZero)).to.equal(ethers.utils.parseEther("1"));
            const balance = await ethers.provider.getBalance(unionBridgeSourceContract.address);
            expect(balance).to.equal(ethers.utils.parseEther("1"));
            const ownerAfterDepositeUNIBalance = await ethers.provider.getBalance(owner.address);

            console.log("before desposite:",ownerBeforeDepositeUNIBalance.toString());
            console.log("aftter desposite:",ownerAfterDepositeUNIBalance.toString());
            expect(ownerBeforeDepositeUNIBalance.gt(ownerAfterDepositeUNIBalance.add(ethers.utils.parseEther("1")))).to.be.true;
            const gasUsed = ownerBeforeDepositeUNIBalance.sub(ownerAfterDepositeUNIBalance).sub(ethers.utils.parseEther("1"));
            console.log("the gas used:",gasUsed.toString());
        });
    });

    describe("desposite erc20",() => {
        it("depost ERC20 value shoud be correct",async () => {
            const params = {
                userAddr:owner.address,
                tokenAddr:token.address,
                receiver:unionBridgeSourceContract.address,
                amount:ethers.utils.parseEther("100"),
                orderId:1,
                chainId:(await ethers.provider.getNetwork()).chainId
            };
            const types = {
                Permit: [
                    { name: "userAddr", type: "address" },
                    { name: "tokenAddr", type: "address" },
                    { name: "receiver", type: "address" },
                    { name: "amount", type: "uint256" },
                    { name: "orderId", type: "uint256" },
                    { name: "chainId", type: "uint256" },
                ],
            };
            const domain = {
                name: "UnionBridgeSource",
                version: "1",
                chainId: (await ethers.provider.getNetwork()).chainId,
                verifyingContract: unionBridgeSourceContract.address,
            };
            const signature = await owner._signTypedData(domain, types, params);
            const data = ethers.utils.defaultAbiCoder.encode(
                [
                    "address",
                   "address",
                   "address",
                   "uint256",
                   "uint256", 
                   "uint256",
                   "bytes"
                ],[
                    params.userAddr,
                    params.tokenAddr,
                    params.receiver,
                    params.amount,
                    params.orderId,
                    params.chainId,
                    signature
                ]
            );
            const ownerBeforeDepositeERC20Balance = await token.balanceOf(owner.address);
            await token.connect(owner).approve(unionBridgeSourceContract.address, ethers.utils.parseEther("100"));
            await unionBridgeSourceContract.connect(owner).depositeERC20(data,{
                gasLimit: 1_000_000,
            });
            expect(await token.balanceOf(unionBridgeSourceContract.address)).to.equal(ethers.utils.parseEther("100"));
        });
    });

    describe("witheDraw UNI",()=>{
        beforeEach("deposite uni",async ()=>{
             const params = {
                userAddr:owner.address,
                receiver:unionBridgeSourceContract.address,
                amount:ethers.utils.parseEther("1"),
                orderId:1,
                chainId:(await ethers.provider.getNetwork()).chainId
            };

            const types = {
                Permit: [
                    { name: "userAddr", type: "address" },
                    { name: "receiver", type: "address" },
                    { name: "amount", type: "uint256" },
                    { name: "orderId", type: "uint256" },
                    { name: "chainId", type: "uint256" },
                ],
            };
            const domain = {
                name: "UnionBridgeSource",
                version: "1",
                chainId: (await ethers.provider.getNetwork()).chainId,
                verifyingContract: unionBridgeSourceContract.address,
            };
            const signature = await owner._signTypedData(domain, types, params);
            const data = ethers.utils.defaultAbiCoder.encode(
                [
                   "address",
                   "address",
                   "uint256",
                   "uint256", 
                   "uint256",
                   "bytes"
                ],[
                    params.userAddr,
                    params.receiver,
                    params.amount,
                    params.orderId,
                    params.chainId,
                    signature
                ]
            );
            const ownerBeforeDepositeUNIBalance = await ethers.provider.getBalance(owner.address);
            await unionBridgeSourceContract.connect(owner).depositeUNI(data,{
                value: ethers.utils.parseEther("1"), 
                gasLimit: 1_000_000,
            });
           

            expect(await unionBridgeSourceContract.balance(ethers.constants.AddressZero)).to.equal(ethers.utils.parseEther("1"));
            const balance = await ethers.provider.getBalance(unionBridgeSourceContract.address);
            expect(balance).to.equal(ethers.utils.parseEther("1"));
            const ownerAfterDepositeUNIBalance = await ethers.provider.getBalance(owner.address);

            console.log("before desposite:",ownerBeforeDepositeUNIBalance.toString());
            console.log("aftter desposite:",ownerAfterDepositeUNIBalance.toString());
            expect(ownerBeforeDepositeUNIBalance.gt(ownerAfterDepositeUNIBalance.add(ethers.utils.parseEther("1")))).to.be.true;
            const gasUsed = ownerBeforeDepositeUNIBalance.sub(ownerAfterDepositeUNIBalance).sub(ethers.utils.parseEther("1"));
            console.log("the gas used:",gasUsed.toString());
        });
        describe("withDraw uni the banlance of owner and reciever shoud be correct",() => {
            it("withDraw uni the banlance of owner and reciever shoud be correct",async () => {
                const params = {
                    caller:owner.address,
                    amount:ethers.utils.parseEther("1"),
                    userAddr:user3.address,
                    orderId:1,
                    chainId:(await ethers.provider.getNetwork()).chainId
                };
                const types = {
                    Permit:[
                        { name: "caller", type: "address" },
                        { name: "amount", type: "uint256" },
                        { name: "userAddr", type: "address"},
                        { name: "orderId", type: "uint256" },
                        { name: "chainId", type: "uint256" },
                    ],
                };
                const domain = {
                    name: "UnionBridgeSource",
                    version: "1",
                    chainId: (await ethers.provider.getNetwork()).chainId,
                    verifyingContract: unionBridgeSourceContract.address,
                };
                const signature = await owner._signTypedData(domain, types, params);
                const data = ethers.utils.defaultAbiCoder.encode(
                    [
                        "address",
                        "uint256",
                        "address",
                        "uint256", 
                        "uint256",
                        "bytes"
                    ],[
                        params.caller,
                        params.amount,
                        params.userAddr,
                        params.orderId,
                        params.chainId,
                        signature
                    ]
                );

                const contractBeforeWithDrawBalance = await ethers.provider.getBalance(unionBridgeSourceContract.address);
                const ownerBeforeWithDrawBalance = await ethers.provider.getBalance(owner.address);
                const user2BeforeWithDrawBalance = await ethers.provider.getBalance(user2.address);
                const user3BeforeWithDrawBalance = await ethers.provider.getBalance(user3.address);

                console.log("contractBeforeWithDrawBalance:",contractBeforeWithDrawBalance);
                console.log("ownerBeforeWithDrawBalance:",ownerBeforeWithDrawBalance);
                console.log("user2BeforeWithDrawBalance:",user2BeforeWithDrawBalance);
                console.log("user3BeforeWithDrawBalance:",user3BeforeWithDrawBalance);
                await unionBridgeSourceContract.withdrawUNI(data,{
                     gasLimit: 1_000_000,
                });

                const contractAfterWithDrawBalance = await ethers.provider.getBalance(unionBridgeSourceContract.address);
                const ownerAfterWithDrawBalance = await ethers.provider.getBalance(owner.address);
                const user2AfterWithDrawBalance = await ethers.provider.getBalance(user2.address);
                const user3AfterWithDrawBalance = await ethers.provider.getBalance(user3.address);
                console.log("contractAfterWithDrawBalance:",contractAfterWithDrawBalance);
                console.log("ownerAfterWithDrawBalance:",ownerAfterWithDrawBalance);
                console.log("user2AfterWithDrawBalance:",user2AfterWithDrawBalance);
                console.log("user3AfterWithDrawBalance:",user3AfterWithDrawBalance);


               const fee =  params.amount.mul(feePercent).div(feeDenominator);
               const userIncome = params.amount.sub(params.amount.mul(feePercent).div(feeDenominator));
               console.log("fee:",fee);
               console.log("userIncome:",userIncome);

               expect(contractAfterWithDrawBalance).to.equal(contractBeforeWithDrawBalance.sub(params.amount));
               expect(user3AfterWithDrawBalance).to.equal(user3BeforeWithDrawBalance.add(userIncome));
            });
        });
       
    });


    describe("withDraw ERC20",()=> {
        beforeEach(async () => {
            const params = {
                userAddr:user1.address,
                tokenAddr:token.address,
                receiver:unionBridgeSourceContract.address,
                amount:ethers.utils.parseEther("100"),
                orderId:1,
                chainId:(await ethers.provider.getNetwork()).chainId
            };
            const types = {
                Permit: [
                    { name: "userAddr", type: "address" },
                    { name: "tokenAddr", type: "address" },
                    { name: "receiver", type: "address" },
                    { name: "amount", type: "uint256" },
                    { name: "orderId", type: "uint256" },
                    { name: "chainId", type: "uint256" },
                ],
            };
            const domain = {
                name: "UnionBridgeSource",
                version: "1",
                chainId: (await ethers.provider.getNetwork()).chainId,
                verifyingContract: unionBridgeSourceContract.address,
            };
            const signature = await owner._signTypedData(domain, types, params);
            const data = ethers.utils.defaultAbiCoder.encode(
                [
                   "address",
                   "address",
                   "address",
                   "uint256",
                   "uint256", 
                   "uint256",
                   "bytes"
                ],[
                    params.userAddr,
                    params.tokenAddr,
                    params.receiver,
                    params.amount,
                    params.orderId,
                    params.chainId,
                    signature
                ]
            );
            const ownerBeforeDepositeERC20Balance = await token.balanceOf(owner.address);
            await token.connect(user1).approve(unionBridgeSourceContract.address, ethers.utils.parseEther("100"));
            await unionBridgeSourceContract.connect(user1).depositeERC20(data,{
                gasLimit: 1_000_000,
            });
            
        });
        describe("withDraw uni the banlance of owner and reciever shoud be correct",()=>{
            it("withDraw uni the banlance of owner and reciever shoud be correct",async () => {
                const params = {
                    caller:owner.address,
                    tokenAddr:token.address,
                    amount:ethers.utils.parseEther("100"),
                    userAddr:user1.address,
                    orderId:1,
                    chainId:(await ethers.provider.getNetwork()).chainId
                };
                const types = {
                    Permit: [
                        { name: "caller", type: "address" },
                        { name: "tokenAddr", type: "address" },
                        { name: "amount", type: "uint256" },
                        { name: "userAddr", type: "address" },
                        { name: "orderId", type: "uint256" },
                        { name: "chainId", type: "uint256" },
                    ],
                };
                const domain = {
                    name: "UnionBridgeSource",
                    version: "1",
                    chainId: (await ethers.provider.getNetwork()).chainId,
                    verifyingContract: unionBridgeSourceContract.address,
                };
                const signature = await owner._signTypedData(domain, types, params);

                const data = ethers.utils.defaultAbiCoder.encode(
                    [
                        "address",
                        "address",
                        "uint256",
                        "address",
                        "uint256", 
                        "uint256",
                        "bytes"
                    ],[
                        params.caller,
                        params.tokenAddr,
                        params.amount,
                        params.userAddr,
                        params.orderId,
                        params.chainId,
                        signature
                    ]
                );

                const contractBeforeWithDrawERC20Balance = await token.balanceOf(unionBridgeSourceContract.address);
                const user1BeforeWithDrawERC20Balance = await token.balanceOf(user1.address);
                const user2BeforeWithDrawERC20Balance = await token.balanceOf(user2.address);
                const ownerBeforeWithDrawERC20Balance = await token.balanceOf(owner.address);

                console.log("contractBeforeWithDrawERC20Balance:",contractBeforeWithDrawERC20Balance);
                console.log("user1BeforeWithDrawERC20Balance:",user1BeforeWithDrawERC20Balance);
                console.log("user2BeforeWithDrawERC20Balance:",user2BeforeWithDrawERC20Balance);
                console.log("ownerBeforeWithDrawERC20Balance:",ownerBeforeWithDrawERC20Balance);

                await unionBridgeSourceContract.connect(owner).withdrawERC20(data,{
                    gasLimit: 1_000_000,
                });

                const contractAfterWithDrawERC20Balance = await token.balanceOf(unionBridgeSourceContract.address);
                const user1AfterWithDrawERC20Balance = await token.balanceOf(user1.address);
                const user2AfterWithDrawERC20Balance = await token.balanceOf(user2.address);
                const ownerAfterWithDrawERC20Balance = await token.balanceOf(owner.address);

                console.log("contractAfterWithDrawERC20Balance:",contractAfterWithDrawERC20Balance);
                console.log("user1AfterWithDrawERC20Balance:",user1AfterWithDrawERC20Balance);
                console.log("user2AfterWithDrawERC20Balance:",user2AfterWithDrawERC20Balance);
                console.log("ownerAfterWithDrawERC20Balance:",ownerAfterWithDrawERC20Balance);

                const fee =  params.amount.mul(feePercent).div(feeDenominator);
                const userIncome = params.amount.sub(params.amount.mul(feePercent).div(feeDenominator));
                console.log("fee:",fee);
                console.log("userIncome:",userIncome);

                expect(user2AfterWithDrawERC20Balance).to.equal(user2BeforeWithDrawERC20Balance.add(fee));
                expect(user1AfterWithDrawERC20Balance).to.equal(user1BeforeWithDrawERC20Balance.add(userIncome));



            });
        });

    });





});


/**
 npx hardhat test ./test/bridge/UnionBridgeSource.test.ts --network ganache
 */