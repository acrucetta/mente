
## Designing API for Humans ([source](https://dev.to/stripe/designing-apis-for-humans-error-messages-94p))

### Object IDs
- Avoid `id = 42`
- Use universally unique identifiers (UUID): ``4c4a82ed-a3e1-4c56-aa0a-26962ddd0425``
- Or even better, add a prefix to them: `pi_3LKQhvGUcADgqoEM3bh6pslE`

### Error messages

Avoid error messages in 200 status.

```
{
  status: 200,
  body: {
    message: "Error"
  }
}
```

>Most error monitoring systems first filter based on status code and then try to parse the body. This error would likely be put in the “everything’s fine” bucket and get completely missed

| **Code**  | **Message**   |
| --------- | ------------- |
| 200 - 299 | All good      |
| 400 - 499 | You messed up |
| 500 - 599 | We messed up  |

E.g. code logic

```javascript
// ❌ Don't forget the error status code
app.post('/your-api-route', async (req, res) => {      
  try {
    // ... your server logic
  } catch (error) {    
    return res.send({ error: { message: error.message } });
  }  

  return res.send('ok');
});

// ✅ Do set the status correctly
app.post('/your-api-route', async (req, res) => {      
  try {
    // ... your server logic
  } catch (error) {    
    return res.status(400).send({ error: { message: error.message } });
  }  

  return res.send('ok');
});
```

**Be helpful with your API errors**

**Bad**
- Which customer?
```
{
  status: 404,
  body: {
    error: {
      message: "Customer not found"
    }    
  }
}
```

**Better**
- Relevant status code
- Clear message
- Error message wrapped in an error object
```
{
  status: 404,
  body: {
    error: {
      message: "Customer cus_Jop8JpEFz1lsCL not found"
    }    
  }
}
```

**Best**
- If you have a type mismatch, say which one
- If your request is missing permissions say which permission method
- If its missing a field, say which field

```
{
  status: 404,
  body: {
    error: {
      message: "Customer cus_Jop8JpEFz1lsCL not found; a similar object exists in live mode, but a test mode key was used to make this request."
    }    
  }
}
```

**Give useful 500 errors when things fail**

Some examples:

- “An error occurred, the team has been informed. If this keeps happening please contact us at `{URL}`”
- “Something went wrong, please check our status page at `{URL}` if this keeps happening”
- “Something goofed, our engineers have been informed. Please try again in a few moments”

**Summary of a good response:**
1. Using the correct HTTP status code
2. Wrapping the error in an “error” object
3. Being helpful by providing:
    1. The error code
    2. The error type
    3. A link to the relevant docs
    4. The API version used in this request
    5. A suggestion on how to fix the issue
4. Providing the request ID to look up the request and response pairing
- 

```
{
  status: 404, -- good status code
  body: {
    error: {
      code: "resource_missing", 
      doc_url: "https://stripe.com/docs/error-codes/resource-missing",
      message: "No such customer: 'cus_Jop8JpEFz1lsCL'; a similar object exists in live mode, but a test mode key was used to make this request.", -- good message
      param: "id",
      type: "invalid_request_error" -- good error type
    }
  },
  headers: {    
    'request-id': 'req_su1OkwzKIeEoCy',
    'stripe-version': '2020-08-27',    
  }  
}
```
### Stripe design patterns

**Use enums vs. booleans**

**Bad**

```
Subscription.canceled={true, false}
Subscription.paused={true, false}
```

**Better**

```
Subscription.status={"active", "canceled", "paused"}
```

**Use nested objects**

**Bad**

```
customer.address_line1 = "Main street 123";
customer.address_city = "San Francisco";
customer.address_postal_code: "12345";
```

**Better**

```
customer.address = {
  line1: "Main Street 123",
  city: "San Francisco",
  postal_code: "12345"
};
```

**Return object types**

```
/v1/customers/:customer/payment_methods/:payment_method

Returns:
{
  "id": "pm_123",
  "object": "payment_method", -- useful for filters
  "created": 1672217299,
  "customer": "cus_123",
  "livemode": false,
  ...
}
```

