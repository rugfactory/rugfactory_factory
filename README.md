# rugfactory contract

this is the rugfactory contract that manages creating tokens and liquidity with rhea (previously ref finance).

![](DOCS/src/rug_banner_100px_B39570.svg)

---

### contracts

ref
- ref-finance-101.testnet
- v2.ref-finance.near

SHIT token - used as payment and reward
- shit-237.factory.v10.meme-cooking.testnet
- shit-1170.meme-cooking.near

wrap near
- wrap.near
- wrap.testnet


rugfactory - this contract
- rugfun.testnet
- rugfun.near

> Note: None of these should be hardcoded in the contract, they are set when you init the conrtact and can be updated at anytime.


![](DOCS/src/rug_banner_100px_B39570.svg)

---

## Building and deploying



```bash
cargo build
cargo near build
./build_cargo.sh
./build_reproducible.sh

cargo check
cargo test
cargo clean


cargo near deploy build-reproducible-wasm <account-id>
near deploy <account-id> <wasm/path>

```

![](DOCS/src/rug_banner_100px_B39570.svg)

---

### rugfactory contract
complete walkthrough of methods and notes

there are some internal methods that the owner can call if necessary, but also get carried out when users call the methods anyone can call. there are also some view methods, just to make it easier for contract to get information for doing some of the other methods.

✅ new
<br/>
needs to be inited with the owner info, ref adress, wrap near adress, and shit token adress
- init

✅ hello
<br/>
we will keep the hello methods, and add a twist where they have to pay 100 Shit to update.
- greeting_get
- greeting_set


✅ user methods
<br/>
the methods for depositing may not be needed, because contract can keep track of users deposit, but they should be able to view their near balance, and withdraw near. will not be able to withdraw shit. and the contract should automatically deduct form a users balance when they do an action.
- user_get_shit_balance, for users to view their shit deposit balance
- user_deposit_near, for users to deposit near
- user_get_near_balance, for users to view their near deposit balance
- user_withdraw_near, for withdrawing near
- user_get_balance, get the near and shit depostite balance of a user.
- ft_on_transfer


✅ token methods and summary of actions
<br/>
this is separate from the whole create form ideas and rug token side of the contract, and this is so users can create tokens if they want to be the full owner. will deduct 1000 $SHIT and 1.99 Near from the users balance when they call this method.
we will probly use this method for the rug side of the contract as well, and it will be nice to take some steps like register with ref and create a new lp when new tokens are created
all tokens are created with conrtact's account being the owener, and then registering the user adress and sending them the tokens
contract will keep track of who created what tokens are created, and who created what idea, so that when the tokens are deleted we can refund the user their money back to their account. only 1.5 near can be refunded to their account not the full amount.
- token_create, for creating a new token, will create a new subaccount send 1.90 near and deplpoy token conrtact and init with new
- token_list_all, for listing all the tokens that have been created
- token_delete, for calling the "rugfactory_token_delete" method on the token account

**ideas summary**
- anyone can submit ideas
- anyone can upvote or downvote
- submitting ideas cost 1.99 near and 1000 $SHIT
- upvoting and down voting cost 1000 $SHIT
- there can only be a max of 10 ideas so adding a new one removed the most downvoted idea
- if an idea does not go on to become a token, their near is returned, the $SHIT is not
- the contract should keep track who submits what ideas
**ideas methods**
- idea_add, submission will contain name, ticker and icon, if icon does not exist will use default icon, and the time they would like their token to live when the token is created and liquidity is added
- idea_get_all, view method for viewing all ideas
- idea_upvote
- idea_downvote
- idea_get_votes, for specific idea
- idea_get_current_idea
- idea_get_creator, view method for getting the creator of a specific idea

**timer summary**
- timer for current token lp
- max amount of time can be 24 hours
- min amount of time 10 minute, so if someone tries to remove time when there is less than an hour, it brings it to 10 minutes.
- no one can add more time when there is less than 5 minuets
- cost 1000 shit to add or remove time
- reward the person who's idea is currently up with 500 $SHIT every time someone adds or removes time.
- a users sets the initial time they would like their project to live when they submit their idea, the timer starts with info form that idea when an idea goes live as a token
timer methods
- timer_get_time, view method for viewing current time left on timer, also shows the full time that the current timer has been running or will run, so a view of a timer that had an initial time of 24 will show 24 hours, how much time left, if time is added or removed the total timer length is updated
- timer_add_time, call method for adding an hour
- timer_remove_time, call method for removing an hour
- timer_get_warning, view method that returns either "🚨 Less than 10 minutes🚨" with actual number of minuets left if less than 10 or "You still have plenty of time"
- timer_reward_idea, internal method for rewarding creator if the current idea when ever anyone calls add or remove time

rugfactory methods summary
- rewards the person who calls this method with a 1000 $SHIT, because someone has to do it.
- this method can only be called when the timer is at 0
- it removes the liquidity for the current idea
- created new token
- lp fee 0.10%, and 100% of new token supply added
rugfactory methods
- rugfactory_the_rug, rugs the current token and creates the next most upvoted idea, and adds liquidity
- rugfactory_lp_add, what happens when adding liquidity, internal method
- rugfactory_lp_remove, what happens when removing lp, internal method
- rugfactory_reward_rugger, intenal method for rewarding rugger 1000 $SHIT
- rugfactory_reward_idea, internal method for rewarding the person of the idea if their idea was alive for more than 30 hours, reward 2000 $SHIT

reward summary
- these are non callable by everyone, these methods are done automatically by the contract when one of the other methods are called

ref methods, and related
for getting details from ref, there should already be wraped near deposited in ref so should not need to do these steps every time. the conrtact should keep track of pool information so it can add and remove liquidity
- ref_view_get_deposit_wrap, Check WNEAR Balance on Ref Finance
- near_wrap, for wraping near
- near_view_wrap_ft_balance_of, Check WNEAR Balance
- ref_wrap_ft_transfer_call, for transfering wrap to ref
- ref_token_ft_transfer_call, for transferring the created token to ref
- ref_call_add_simple_pool, for creating a pool with the latest token
- ref_call_add_liquidity, for adding liquidity to the newly created pool
- ref_view_get_pool, for viewing info of the current pool
- ref_call_remove_liquidity, for removing all the liquidity for current token lp
- ref_call_storage_deposit, for registering account, account should be already registered, so this will not happen automatically but owner can call this
- ref_call_register_tokens, for registering tokens with ref



✅ admin
<br/>
- admin_get_info, returns contract configuration information including owner_id, contract addresses
- admin_update_info, to update info
- admin_get_balance, for returning the near balance available for contract owner that is not taken out by storage or is not users balances

---

🤑 Prices and fees :: to keep track of what i am chaing people
- charge user 1000 $SHIT for creating a token
- charge user 1.99 Near for creating a token, and 1000 $SHIT
- charge user 1000 $SHIT for submiting an idea
- charge user 1000 $SHIT for adding time
- charge user 1000 $SHIT for removing time
- charge user 100 $SHIT for updating greeting
- internal: send 1.95 Near to the new token account when creating token

ℹ️ Other info in case I forgot to mention it elsewheer
- 24 deciamls satndard for token creation
- 1 billion token supply
- default icon, and max icon size 1KB
- 0.10% fee for lp pair


---
![](DOCS/src/rug_banner_100px_B39570.svg)

copyright: 2025 by sleet.near, in partnership with huggies.near
