<?xml version="1.0" encoding="UTF-8"?>
<WebElementEntity>
   <description></description>
   <name>div_Join Practo                            _a77d58</name>
   <tag></tag>
   <elementGuidId>3c8a5494-960f-4719-8a86-c8c599f0381b</elementGuidId>
   <selectorCollection>
      <entry>
         <key>CSS</key>
         <value>#variableHeightContainer</value>
      </entry>
      <entry>
         <key>XPATH</key>
         <value>//div[@id='variableHeightContainer']</value>
      </entry>
   </selectorCollection>
   <selectorMethod>XPATH</selectorMethod>
   <smartLocatorCollection>
      <entry>
         <key>SMART_LOCATOR</key>
         <value>#variableHeightContainer</value>
      </entry>
   </smartLocatorCollection>
   <smartLocatorEnabled>false</smartLocatorEnabled>
   <useRalativeImagePath>true</useRalativeImagePath>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>tag</name>
      <type>Main</type>
      <value>div</value>
      <webElementGuid>990f81fa-0504-4b02-8fb9-e824058c0779</webElementGuid>
   </webElementProperties>
   <webElementProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>id</name>
      <type>Main</type>
      <value>variableHeightContainer</value>
      <webElementGuid>f2a41802-16b7-4c98-ad6d-2f2374fd3c99</webElementGuid>
   </webElementProperties>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>class</name>
      <type>Main</type>
      <value>variableHeightContainer variableHeightContainer-fit</value>
      <webElementGuid>38ec6c70-22f4-492e-b3a6-50c1dcaeb296</webElementGuid>
   </webElementProperties>
   <webElementProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>text</name>
      <type>Main</type>
      <value>
        
        
      
      



  




  
   
      
        
          Join Practo
          
              
          
                Register Here
          
          Are you a doctor? 
          
        
      
   
   
    Full Name
    
        
         
  

    
    

    
        Mobile Number
        
            +91(IND)+65(SGP)+63(PHL)+60(MYS)+62(IDN)+55(BRA)+52(MEX)+54(ARG)+56(CHL)+84(VNM)+971(UAE)+965(KW)+255(TZA)+973(BH)+966(SA)+1(USA)
            
              
            
        
        
    

    
    
        Create Password
        
            
            
              Password Strength: Weak
            
             
  

        
    

    
      
      
      
      
      
   

   
      
        
         
            
            
         
         Receive relevant offers and promotional communication from Practo
         
             
                 
                 By signing up, I agree to terms
                 
             
         
        
      
      
      
        
        
        Send OTP
      
   
  
 




  


  #navCollapse,
 #bottomNavCollapse {
     background-color: #FFFFFF;
 }
 nav[role='navigation'] {
     box-shadow: none;
     border: none;
 }
 .navbar-header {
     background-color: white;
 }
 .hrContainer {
     margin-top: 1%;
     margin-bottom: 1%;
     width: 410px;
 }
 .fbMsg {
  position: relative;
  top: -10px;
 }
 .join-practo {
   visibility: hidden;
 }
 #registerLink {
    color: #14bef0;
    font-weight: bold;
}
li > a#registerLink {
    border-bottom: 3px solid #14bef0;
  }
 #registerLink:hover {
  text-decoration: none;
 }
 .align-top {
  margin-top: 3px;
 }


window.registerCaptchaWidget = new RecaptchaWidget(
  'recaptcha-container-register',
  function (recaptchaResponse) {
    document.getElementById('register_email_form').submit();
  }
);
  var registerEmailFn  =  function(event){
       var mobile = document.getElementById(&quot;mobile&quot;);
       var email = document.getElementById(&quot;email&quot;);
       var password =  document.getElementById(&quot;password&quot;);
       var name =  document.getElementById(&quot;name&quot;);
       var isPatientSignup = document.getElementById('is-patient-signup');
       if (mobile) {
        var mobileValidation = PRC.validator.Empty(mobile, null, 'placeholder');
       };
       if (email) {
        var emailValidation = PRC.validator.Empty(email, null, 'placeholder');
       }
       var passwordValidation = PRC.validator.Empty(password, null, 'placeholder');
       var nameErrorBlock = document.getElementById(&quot;nameErrorBlock&quot;);
       var mobileErrorBlock = document.getElementById(&quot;mobileErrorBlock&quot;);
       var emailErrorBlock = document.getElementById(&quot;emailErrorBlock&quot;);
       var passwordErrorBlock = document.getElementById(&quot;passwordErrorBlock&quot;);
       var hasError = false;

       if( typeof mobileValidation != &quot;undefined&quot; &amp;&amp; !mobileValidation){
           if(PRC.settings.gaTrackingCode  &amp;&amp; PRC.settings.gaTrackingCode.buttonClick){


              var pathName = window.location.pathname;
                if (pathName.indexOf('patient')>-1) {
                    PRC.settings.gaTrackingCode.buttonClick(&quot;PatRegFail_Email&quot;);
                  }else if(pathName.indexOf('doctor')>-1){
                      PRC.settings.gaTrackingCode.buttonClick(&quot;DocRegFail_Email&quot;);
                  }

          }

           mobileErrorBlock.innerHTML = mobile.errors[0];
           mobileErrorBlock.style.display=&quot;block&quot;;
           hasError = true;
           event.preventDefault();
       }
       else
          (typeof mobileErrorBlock != &quot;undefined&quot; &amp;&amp; mobileErrorBlock) ? mobileErrorBlock.style.cssText = &quot; &quot; : &quot;&quot;;

        if( typeof emailValidation != &quot;undefined&quot; &amp;&amp; !emailValidation){
           if(PRC.settings.gaTrackingCode  &amp;&amp; PRC.settings.gaTrackingCode.buttonClick){


              var pathName = window.location.pathname;
                if (pathName.indexOf('patient')>-1) {
                    PRC.settings.gaTrackingCode.buttonClick(&quot;PatRegFail_Email&quot;);
                  }else if(pathName.indexOf('doctor')>-1){
                      PRC.settings.gaTrackingCode.buttonClick(&quot;DocRegFail_Email&quot;);
                  }

          }

           emailErrorBlock.innerHTML = email.errors[0];
           emailErrorBlock.style.display=&quot;block&quot;;
           hasError = true;
           event.preventDefault();
       }
       else
          (typeof emailErrorBlock != &quot;undefined&quot; &amp;&amp; emailErrorBlock) ? emailErrorBlock.style.cssText = &quot; &quot; : &quot;&quot;;

        if(isPatientSignup) {
           var nameValidation = PRC.validator.Empty(name, null, 'placeholder');
           if(!nameValidation){
              nameErrorBlock.innerHTML = name.errors[0];
              nameErrorBlock.style.display=&quot;block&quot;;
              hasError = true;
              event.preventDefault();
           }
           else {
              name.value = name.value.trim();
              nameErrorBlock.innerHTML = '';
              nameErrorBlock.style.cssText = &quot; &quot;;
           }
           name.errors= [];
        }
       if(!passwordValidation){

              if(PRC.settings.gaTrackingCode  &amp;&amp; PRC.settings.gaTrackingCode.buttonClick){
                 var pathName = window.location.pathname;
                  if (pathName.indexOf('patient')>-1) {
                      PRC.settings.gaTrackingCode.buttonClick(&quot;PatRegFail_Email&quot;);
                    }else if(pathName.indexOf('doctor')>-1){
                        PRC.settings.gaTrackingCode.buttonClick(&quot;DocRegFail_Email&quot;);
                    }

              }

              passwordErrorBlock.innerHTML = password.errors[0];
              passwordErrorBlock.style.display=&quot;block&quot;;

              if(event.preventDefault)
                event.preventDefault();
              else{
                // event for enter button since event is nt associated for window object
                event.defaultPrevented =  true;
              }
              hasError = true;
       }
       else
        passwordErrorBlock.style.cssText = &quot; &quot;;

        if (!hasError) {
          var pathName = window.location.pathname;
          if (PRC.settings.gaTrackingCode  &amp;&amp; PRC.settings.gaTrackingCode.buttonClick) {
            if (pathName.indexOf('patient')>-1) {
              PRC.settings.gaTrackingCode.buttonClick(&quot;PatReg_Email&quot;);
            } else if(pathName.indexOf('doctor')>-1){
              PRC.settings.gaTrackingCode.buttonClick(&quot;DocReg_Email&quot;);
            }
          }
          // PEL -- Start
          if (typeof pel != 'undefined' &amp;&amp; typeof pel.tracker != 'undefined') {
            var objectContext = { 'Login Type': 'Phone' };
            pel.logEvent('Login', 'Interacted', '', objectContext);

            objectContext = {
              'Type': 'Notification',
              'State': $('#subscribe_promotion').prop('checked') === true ? 'Yes' : 'No',
              'Is Logged In': 'No'
            };
            pel.logEvent('User Setting', 'Updated', '', objectContext);
          }
          // PEL -- End
          event.preventDefault();
          window.registerCaptchaWidget.render();
       }
       (mobile) ? mobile.errors= [] : &quot;&quot;;
       (email) ? email.errors= []: &quot;&quot;;
       password.errors= [];

       event.stopPropagation();
   };

 var checkMobileFn =  function(event){
         var mobile = document.getElementById(&quot;mobile&quot;);
         if (mobile) {
           var msg = &quot;Invalid Mobile Number&quot;;
           if(mobile.value.trim().length){
                var errorBlock =  document.getElementById(&quot;mobileErrorBlock&quot;);
              if(!PRC.validator.Empty(mobile, msg, 'placeholder') ){
                errorBlock.innerHTML = msg;
                errorBlock.style.display = &quot;block&quot;;

              }else{
                   errorBlock.innerHTML = &quot;&quot;;
                   errorBlock.style.display = &quot;none&quot;;

              }
              mobile.errors = [];
              event.preventDefault();
           }

           event.stopPropagation();
          }
   };

   var doctor  = function(event){
        if(PRC.settings.gaTrackingCode  &amp;&amp; PRC.settings.gaTrackingCode.linkClick){
            PRC.settings.gaTrackingCode.linkClick(&quot;DocReg_ForkFromPat&quot;, function(){
              window.location.replace('/doctor_signup' + window.location.search);
            });
         }
         event.stopPropagation();
         event.preventDefault();
   };

   var notDoctor = function(event) {
             if(PRC.settings.gaTrackingCode  &amp;&amp; PRC.settings.gaTrackingCode.linkClick){
              PRC.settings.gaTrackingCode.linkClick(&quot;PatReg_ForkFromDoc&quot;, function(){
                var url = '/new_patient_signup' + window.location.search;
                window.location.replace(url.replace('intent=software', 'intent=accounts'));
              });
           }
          event.stopPropagation();
          event.preventDefault();
   };

   var terms = function(event){
   };


   var obj = {
      init: function(eleIdArr){
            if(eleIdArr &amp;&amp; eleIdArr.length){
             var length = eleIdArr.length;
             for(var iloop=0; length--;iloop++){
                 var eleObj = eleIdArr[iloop];
                 if(document.getElementById(eleObj.eleId))
                     document.getElementById(eleObj.eleId).addEventListener(eleObj.event, this, eleObj.bubble?eleObj.bubble:false);
             }
           }

          window.addEventListener(&quot;keypress&quot;, function(event){
                if(event.keyCode == 13){
                  var registerEmailBtn =  document.getElementById(&quot;EmailRegister&quot;);
                  registerEmailBtn.onclick = registerEmailFn;
                  registerEmailBtn.onclick.apply(registerEmailBtn,[event]);
                }

          } , false);

      }
      ,handleEvent: function(event){
          switch(event.target.id){
              case &quot;EmailRegister&quot;:
                                    registerEmailFn(event);
                                    break;
              case &quot;EmailRegisterImage&quot;:
                                    registerEmailFn(event);
                                    break;

              case &quot;mobile&quot;        :
                                    checkMobileFn(event);
                                    break;

              case &quot;doctor&quot;        :
                                    doctor(event);
                                    break;

              case &quot;notDoctor&quot;     :
                                    notDoctor(event);
                                    break;
              case &quot;terms&quot;     :
                                    terms(event);
                                    break;

          };

      }
   };


   (function(eventObj){

      var eleArr = [
                       {&quot;eleId&quot;: &quot;EmailRegister&quot;, &quot;event&quot;: &quot;click&quot;},
                       {&quot;eleId&quot;: &quot;EmailRegisterImage&quot;, &quot;event&quot;: &quot;click&quot;},
                       {&quot;eleId&quot;: &quot;EmailRegister&quot;, &quot;event&quot;: &quot;click&quot;},
                       {&quot;eleId&quot;: &quot;EmailRegister&quot;, &quot;event&quot;: &quot;click&quot;},
                       {&quot;eleId&quot;: &quot;email&quot;, &quot;event&quot;: &quot;blur&quot;},
                       {&quot;eleId&quot;: &quot;doctor&quot;, &quot;event&quot;: &quot;click&quot;},
                       {&quot;eleId&quot;: &quot;notDoctor&quot;, &quot;event&quot;: &quot;click&quot;},
                       {&quot;eleId&quot;: &quot;mobile&quot;, &quot;event&quot;: &quot;blur&quot;},
                       {&quot;eleId&quot;: &quot;terms&quot;, &quot;event&quot;: &quot;click&quot;}
                     ];
      eventObj.init(eleArr);


    if(window.opener &amp;&amp; window.opener.PRC){
        window.opener.PRC.fbPopupFlag = true;
        window.opener.PRC.redirectURL = window.location.href.split('#')[0];
    }

   })(obj)
   if($(&quot;#mobileErrorBlock&quot;).text().trim()){
     $('#mobile').addClass('borderRed');
   }
   var joinPractoDoctor = document.getElementById('joinPractoDoctor');
   if(joinPractoDoctor ){
     var joinPractoDoctorHtml = joinPractoDoctor.innerHTML;
     if(joinPractoDoctorHtml.length > 21) {
       $('.box-header').css({
         &quot;font-size&quot;: &quot;14px&quot;
       });
     }
   }






$(function () {
  $('#password').on('input', function() {
    var password = $(this).val();
    if (password &amp;&amp; password.length &lt;= 72 &amp;&amp; password.length > 0 &amp;&amp; typeof zxcvbn !== &quot;undefined&quot;) {
      var user_dictionary = [
          'practo',
          $('#mobile').val()
      ];
      Array.prototype.push.apply(user_dictionary, $('#name').val().split(' '));
      var result = zxcvbn(password, user_dictionary);
      var strength_label = [
        ['very-weak', &quot;Very Weak&quot;],
        ['weak', &quot;Weak&quot;],
        ['moderate', &quot;Moderate&quot;],
        ['strong', &quot;Strong&quot;],
        ['very-strong', &quot;Very Strong&quot;]
      ][result.score];
      $('#password-strength-indicator').removeClass();
      $('#password-strength-indicator').addClass(strength_label[0]);
      $('#password-strength-indicator').html(strength_label[1]);
      $('.password-strength').css('display', 'block');
    } else {
      $('.password-strength').css('display', 'none');
    }
  });
});


  
    $('#name,#mobile,#password').click(function(){
      if ($('#country').val() === 'IN'){
        // Nav will not present inside an iframe
        if (window.self === window.top) {
          $('body').practoNav('attemptTruecallerLogin', {});
        }
      }
    });
  


      
        window.emptyMsg = &quot;field cannot be empty&quot;;
      
         
      </value>
      <webElementGuid>4a994b94-9c29-4bfd-9167-3b8af711dfeb</webElementGuid>
   </webElementProperties>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath</name>
      <type>Main</type>
      <value>id(&quot;variableHeightContainer&quot;)</value>
      <webElementGuid>ad8f3165-e7e8-4d9c-bbdc-d06f9ceaf3c5</webElementGuid>
   </webElementProperties>
   <webElementXpaths>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:attributes</name>
      <type>Main</type>
      <value>//div[@id='variableHeightContainer']</value>
      <webElementGuid>10300baa-861c-4625-a69b-eb81be77e86c</webElementGuid>
   </webElementXpaths>
   <webElementXpaths>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:idRelative</name>
      <type>Main</type>
      <value>//div[@id='spSignup']/div[3]</value>
      <webElementGuid>80c2d17a-44c6-42a6-a490-8181fc6924cb</webElementGuid>
   </webElementXpaths>
   <webElementXpaths>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:neighbor</name>
      <type>Main</type>
      <value>(.//*[normalize-space(text()) and normalize-space(.)='Login / Signup'])[1]/following::div[4]</value>
      <webElementGuid>cbbaeabf-38cf-4a6e-8a60-0b716fb03b2d</webElementGuid>
   </webElementXpaths>
   <webElementXpaths>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:neighbor</name>
      <type>Main</type>
      <value>(.//*[normalize-space(text()) and normalize-space(.)='Help'])[2]/following::div[5]</value>
      <webElementGuid>dd4e9cb5-1785-433c-abbe-34da9ae897ab</webElementGuid>
   </webElementXpaths>
   <webElementXpaths>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:position</name>
      <type>Main</type>
      <value>//div[2]/div[3]</value>
      <webElementGuid>2bd09f76-b954-4e59-88b7-71ca89240b69</webElementGuid>
   </webElementXpaths>
   <webElementXpaths>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:customAttributes</name>
      <type>Main</type>
      <value>//div[@id = 'variableHeightContainer' and (text() = concat(&quot;
        
        
      
      



  




  
   
      
        
          Join Practo
          
              
          
                Register Here
          
          Are you a doctor? 
          
        
      
   
   
    Full Name
    
        
         
  

    
    

    
        Mobile Number
        
            +91(IND)+65(SGP)+63(PHL)+60(MYS)+62(IDN)+55(BRA)+52(MEX)+54(ARG)+56(CHL)+84(VNM)+971(UAE)+965(KW)+255(TZA)+973(BH)+966(SA)+1(USA)
            
              
            
        
        
    

    
    
        Create Password
        
            
            
              Password Strength: Weak
            
             
  

        
    

    
      
      
      
      
      
   

   
      
        
         
            
            
         
         Receive relevant offers and promotional communication from Practo
         
             
                 
                 By signing up, I agree to terms
                 
             
         
        
      
      
      
        
        
        Send OTP
      
   
  
 




  


  #navCollapse,
 #bottomNavCollapse {
     background-color: #FFFFFF;
 }
 nav[role=&quot; , &quot;'&quot; , &quot;navigation&quot; , &quot;'&quot; , &quot;] {
     box-shadow: none;
     border: none;
 }
 .navbar-header {
     background-color: white;
 }
 .hrContainer {
     margin-top: 1%;
     margin-bottom: 1%;
     width: 410px;
 }
 .fbMsg {
  position: relative;
  top: -10px;
 }
 .join-practo {
   visibility: hidden;
 }
 #registerLink {
    color: #14bef0;
    font-weight: bold;
}
li > a#registerLink {
    border-bottom: 3px solid #14bef0;
  }
 #registerLink:hover {
  text-decoration: none;
 }
 .align-top {
  margin-top: 3px;
 }


window.registerCaptchaWidget = new RecaptchaWidget(
  &quot; , &quot;'&quot; , &quot;recaptcha-container-register&quot; , &quot;'&quot; , &quot;,
  function (recaptchaResponse) {
    document.getElementById(&quot; , &quot;'&quot; , &quot;register_email_form&quot; , &quot;'&quot; , &quot;).submit();
  }
);
  var registerEmailFn  =  function(event){
       var mobile = document.getElementById(&quot;mobile&quot;);
       var email = document.getElementById(&quot;email&quot;);
       var password =  document.getElementById(&quot;password&quot;);
       var name =  document.getElementById(&quot;name&quot;);
       var isPatientSignup = document.getElementById(&quot; , &quot;'&quot; , &quot;is-patient-signup&quot; , &quot;'&quot; , &quot;);
       if (mobile) {
        var mobileValidation = PRC.validator.Empty(mobile, null, &quot; , &quot;'&quot; , &quot;placeholder&quot; , &quot;'&quot; , &quot;);
       };
       if (email) {
        var emailValidation = PRC.validator.Empty(email, null, &quot; , &quot;'&quot; , &quot;placeholder&quot; , &quot;'&quot; , &quot;);
       }
       var passwordValidation = PRC.validator.Empty(password, null, &quot; , &quot;'&quot; , &quot;placeholder&quot; , &quot;'&quot; , &quot;);
       var nameErrorBlock = document.getElementById(&quot;nameErrorBlock&quot;);
       var mobileErrorBlock = document.getElementById(&quot;mobileErrorBlock&quot;);
       var emailErrorBlock = document.getElementById(&quot;emailErrorBlock&quot;);
       var passwordErrorBlock = document.getElementById(&quot;passwordErrorBlock&quot;);
       var hasError = false;

       if( typeof mobileValidation != &quot;undefined&quot; &amp;&amp; !mobileValidation){
           if(PRC.settings.gaTrackingCode  &amp;&amp; PRC.settings.gaTrackingCode.buttonClick){


              var pathName = window.location.pathname;
                if (pathName.indexOf(&quot; , &quot;'&quot; , &quot;patient&quot; , &quot;'&quot; , &quot;)>-1) {
                    PRC.settings.gaTrackingCode.buttonClick(&quot;PatRegFail_Email&quot;);
                  }else if(pathName.indexOf(&quot; , &quot;'&quot; , &quot;doctor&quot; , &quot;'&quot; , &quot;)>-1){
                      PRC.settings.gaTrackingCode.buttonClick(&quot;DocRegFail_Email&quot;);
                  }

          }

           mobileErrorBlock.innerHTML = mobile.errors[0];
           mobileErrorBlock.style.display=&quot;block&quot;;
           hasError = true;
           event.preventDefault();
       }
       else
          (typeof mobileErrorBlock != &quot;undefined&quot; &amp;&amp; mobileErrorBlock) ? mobileErrorBlock.style.cssText = &quot; &quot; : &quot;&quot;;

        if( typeof emailValidation != &quot;undefined&quot; &amp;&amp; !emailValidation){
           if(PRC.settings.gaTrackingCode  &amp;&amp; PRC.settings.gaTrackingCode.buttonClick){


              var pathName = window.location.pathname;
                if (pathName.indexOf(&quot; , &quot;'&quot; , &quot;patient&quot; , &quot;'&quot; , &quot;)>-1) {
                    PRC.settings.gaTrackingCode.buttonClick(&quot;PatRegFail_Email&quot;);
                  }else if(pathName.indexOf(&quot; , &quot;'&quot; , &quot;doctor&quot; , &quot;'&quot; , &quot;)>-1){
                      PRC.settings.gaTrackingCode.buttonClick(&quot;DocRegFail_Email&quot;);
                  }

          }

           emailErrorBlock.innerHTML = email.errors[0];
           emailErrorBlock.style.display=&quot;block&quot;;
           hasError = true;
           event.preventDefault();
       }
       else
          (typeof emailErrorBlock != &quot;undefined&quot; &amp;&amp; emailErrorBlock) ? emailErrorBlock.style.cssText = &quot; &quot; : &quot;&quot;;

        if(isPatientSignup) {
           var nameValidation = PRC.validator.Empty(name, null, &quot; , &quot;'&quot; , &quot;placeholder&quot; , &quot;'&quot; , &quot;);
           if(!nameValidation){
              nameErrorBlock.innerHTML = name.errors[0];
              nameErrorBlock.style.display=&quot;block&quot;;
              hasError = true;
              event.preventDefault();
           }
           else {
              name.value = name.value.trim();
              nameErrorBlock.innerHTML = &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;;
              nameErrorBlock.style.cssText = &quot; &quot;;
           }
           name.errors= [];
        }
       if(!passwordValidation){

              if(PRC.settings.gaTrackingCode  &amp;&amp; PRC.settings.gaTrackingCode.buttonClick){
                 var pathName = window.location.pathname;
                  if (pathName.indexOf(&quot; , &quot;'&quot; , &quot;patient&quot; , &quot;'&quot; , &quot;)>-1) {
                      PRC.settings.gaTrackingCode.buttonClick(&quot;PatRegFail_Email&quot;);
                    }else if(pathName.indexOf(&quot; , &quot;'&quot; , &quot;doctor&quot; , &quot;'&quot; , &quot;)>-1){
                        PRC.settings.gaTrackingCode.buttonClick(&quot;DocRegFail_Email&quot;);
                    }

              }

              passwordErrorBlock.innerHTML = password.errors[0];
              passwordErrorBlock.style.display=&quot;block&quot;;

              if(event.preventDefault)
                event.preventDefault();
              else{
                // event for enter button since event is nt associated for window object
                event.defaultPrevented =  true;
              }
              hasError = true;
       }
       else
        passwordErrorBlock.style.cssText = &quot; &quot;;

        if (!hasError) {
          var pathName = window.location.pathname;
          if (PRC.settings.gaTrackingCode  &amp;&amp; PRC.settings.gaTrackingCode.buttonClick) {
            if (pathName.indexOf(&quot; , &quot;'&quot; , &quot;patient&quot; , &quot;'&quot; , &quot;)>-1) {
              PRC.settings.gaTrackingCode.buttonClick(&quot;PatReg_Email&quot;);
            } else if(pathName.indexOf(&quot; , &quot;'&quot; , &quot;doctor&quot; , &quot;'&quot; , &quot;)>-1){
              PRC.settings.gaTrackingCode.buttonClick(&quot;DocReg_Email&quot;);
            }
          }
          // PEL -- Start
          if (typeof pel != &quot; , &quot;'&quot; , &quot;undefined&quot; , &quot;'&quot; , &quot; &amp;&amp; typeof pel.tracker != &quot; , &quot;'&quot; , &quot;undefined&quot; , &quot;'&quot; , &quot;) {
            var objectContext = { &quot; , &quot;'&quot; , &quot;Login Type&quot; , &quot;'&quot; , &quot;: &quot; , &quot;'&quot; , &quot;Phone&quot; , &quot;'&quot; , &quot; };
            pel.logEvent(&quot; , &quot;'&quot; , &quot;Login&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;Interacted&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;, objectContext);

            objectContext = {
              &quot; , &quot;'&quot; , &quot;Type&quot; , &quot;'&quot; , &quot;: &quot; , &quot;'&quot; , &quot;Notification&quot; , &quot;'&quot; , &quot;,
              &quot; , &quot;'&quot; , &quot;State&quot; , &quot;'&quot; , &quot;: $(&quot; , &quot;'&quot; , &quot;#subscribe_promotion&quot; , &quot;'&quot; , &quot;).prop(&quot; , &quot;'&quot; , &quot;checked&quot; , &quot;'&quot; , &quot;) === true ? &quot; , &quot;'&quot; , &quot;Yes&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;No&quot; , &quot;'&quot; , &quot;,
              &quot; , &quot;'&quot; , &quot;Is Logged In&quot; , &quot;'&quot; , &quot;: &quot; , &quot;'&quot; , &quot;No&quot; , &quot;'&quot; , &quot;
            };
            pel.logEvent(&quot; , &quot;'&quot; , &quot;User Setting&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;Updated&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;, objectContext);
          }
          // PEL -- End
          event.preventDefault();
          window.registerCaptchaWidget.render();
       }
       (mobile) ? mobile.errors= [] : &quot;&quot;;
       (email) ? email.errors= []: &quot;&quot;;
       password.errors= [];

       event.stopPropagation();
   };

 var checkMobileFn =  function(event){
         var mobile = document.getElementById(&quot;mobile&quot;);
         if (mobile) {
           var msg = &quot;Invalid Mobile Number&quot;;
           if(mobile.value.trim().length){
                var errorBlock =  document.getElementById(&quot;mobileErrorBlock&quot;);
              if(!PRC.validator.Empty(mobile, msg, &quot; , &quot;'&quot; , &quot;placeholder&quot; , &quot;'&quot; , &quot;) ){
                errorBlock.innerHTML = msg;
                errorBlock.style.display = &quot;block&quot;;

              }else{
                   errorBlock.innerHTML = &quot;&quot;;
                   errorBlock.style.display = &quot;none&quot;;

              }
              mobile.errors = [];
              event.preventDefault();
           }

           event.stopPropagation();
          }
   };

   var doctor  = function(event){
        if(PRC.settings.gaTrackingCode  &amp;&amp; PRC.settings.gaTrackingCode.linkClick){
            PRC.settings.gaTrackingCode.linkClick(&quot;DocReg_ForkFromPat&quot;, function(){
              window.location.replace(&quot; , &quot;'&quot; , &quot;/doctor_signup&quot; , &quot;'&quot; , &quot; + window.location.search);
            });
         }
         event.stopPropagation();
         event.preventDefault();
   };

   var notDoctor = function(event) {
             if(PRC.settings.gaTrackingCode  &amp;&amp; PRC.settings.gaTrackingCode.linkClick){
              PRC.settings.gaTrackingCode.linkClick(&quot;PatReg_ForkFromDoc&quot;, function(){
                var url = &quot; , &quot;'&quot; , &quot;/new_patient_signup&quot; , &quot;'&quot; , &quot; + window.location.search;
                window.location.replace(url.replace(&quot; , &quot;'&quot; , &quot;intent=software&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;intent=accounts&quot; , &quot;'&quot; , &quot;));
              });
           }
          event.stopPropagation();
          event.preventDefault();
   };

   var terms = function(event){
   };


   var obj = {
      init: function(eleIdArr){
            if(eleIdArr &amp;&amp; eleIdArr.length){
             var length = eleIdArr.length;
             for(var iloop=0; length--;iloop++){
                 var eleObj = eleIdArr[iloop];
                 if(document.getElementById(eleObj.eleId))
                     document.getElementById(eleObj.eleId).addEventListener(eleObj.event, this, eleObj.bubble?eleObj.bubble:false);
             }
           }

          window.addEventListener(&quot;keypress&quot;, function(event){
                if(event.keyCode == 13){
                  var registerEmailBtn =  document.getElementById(&quot;EmailRegister&quot;);
                  registerEmailBtn.onclick = registerEmailFn;
                  registerEmailBtn.onclick.apply(registerEmailBtn,[event]);
                }

          } , false);

      }
      ,handleEvent: function(event){
          switch(event.target.id){
              case &quot;EmailRegister&quot;:
                                    registerEmailFn(event);
                                    break;
              case &quot;EmailRegisterImage&quot;:
                                    registerEmailFn(event);
                                    break;

              case &quot;mobile&quot;        :
                                    checkMobileFn(event);
                                    break;

              case &quot;doctor&quot;        :
                                    doctor(event);
                                    break;

              case &quot;notDoctor&quot;     :
                                    notDoctor(event);
                                    break;
              case &quot;terms&quot;     :
                                    terms(event);
                                    break;

          };

      }
   };


   (function(eventObj){

      var eleArr = [
                       {&quot;eleId&quot;: &quot;EmailRegister&quot;, &quot;event&quot;: &quot;click&quot;},
                       {&quot;eleId&quot;: &quot;EmailRegisterImage&quot;, &quot;event&quot;: &quot;click&quot;},
                       {&quot;eleId&quot;: &quot;EmailRegister&quot;, &quot;event&quot;: &quot;click&quot;},
                       {&quot;eleId&quot;: &quot;EmailRegister&quot;, &quot;event&quot;: &quot;click&quot;},
                       {&quot;eleId&quot;: &quot;email&quot;, &quot;event&quot;: &quot;blur&quot;},
                       {&quot;eleId&quot;: &quot;doctor&quot;, &quot;event&quot;: &quot;click&quot;},
                       {&quot;eleId&quot;: &quot;notDoctor&quot;, &quot;event&quot;: &quot;click&quot;},
                       {&quot;eleId&quot;: &quot;mobile&quot;, &quot;event&quot;: &quot;blur&quot;},
                       {&quot;eleId&quot;: &quot;terms&quot;, &quot;event&quot;: &quot;click&quot;}
                     ];
      eventObj.init(eleArr);


    if(window.opener &amp;&amp; window.opener.PRC){
        window.opener.PRC.fbPopupFlag = true;
        window.opener.PRC.redirectURL = window.location.href.split(&quot; , &quot;'&quot; , &quot;#&quot; , &quot;'&quot; , &quot;)[0];
    }

   })(obj)
   if($(&quot;#mobileErrorBlock&quot;).text().trim()){
     $(&quot; , &quot;'&quot; , &quot;#mobile&quot; , &quot;'&quot; , &quot;).addClass(&quot; , &quot;'&quot; , &quot;borderRed&quot; , &quot;'&quot; , &quot;);
   }
   var joinPractoDoctor = document.getElementById(&quot; , &quot;'&quot; , &quot;joinPractoDoctor&quot; , &quot;'&quot; , &quot;);
   if(joinPractoDoctor ){
     var joinPractoDoctorHtml = joinPractoDoctor.innerHTML;
     if(joinPractoDoctorHtml.length > 21) {
       $(&quot; , &quot;'&quot; , &quot;.box-header&quot; , &quot;'&quot; , &quot;).css({
         &quot;font-size&quot;: &quot;14px&quot;
       });
     }
   }






$(function () {
  $(&quot; , &quot;'&quot; , &quot;#password&quot; , &quot;'&quot; , &quot;).on(&quot; , &quot;'&quot; , &quot;input&quot; , &quot;'&quot; , &quot;, function() {
    var password = $(this).val();
    if (password &amp;&amp; password.length &lt;= 72 &amp;&amp; password.length > 0 &amp;&amp; typeof zxcvbn !== &quot;undefined&quot;) {
      var user_dictionary = [
          &quot; , &quot;'&quot; , &quot;practo&quot; , &quot;'&quot; , &quot;,
          $(&quot; , &quot;'&quot; , &quot;#mobile&quot; , &quot;'&quot; , &quot;).val()
      ];
      Array.prototype.push.apply(user_dictionary, $(&quot; , &quot;'&quot; , &quot;#name&quot; , &quot;'&quot; , &quot;).val().split(&quot; , &quot;'&quot; , &quot; &quot; , &quot;'&quot; , &quot;));
      var result = zxcvbn(password, user_dictionary);
      var strength_label = [
        [&quot; , &quot;'&quot; , &quot;very-weak&quot; , &quot;'&quot; , &quot;, &quot;Very Weak&quot;],
        [&quot; , &quot;'&quot; , &quot;weak&quot; , &quot;'&quot; , &quot;, &quot;Weak&quot;],
        [&quot; , &quot;'&quot; , &quot;moderate&quot; , &quot;'&quot; , &quot;, &quot;Moderate&quot;],
        [&quot; , &quot;'&quot; , &quot;strong&quot; , &quot;'&quot; , &quot;, &quot;Strong&quot;],
        [&quot; , &quot;'&quot; , &quot;very-strong&quot; , &quot;'&quot; , &quot;, &quot;Very Strong&quot;]
      ][result.score];
      $(&quot; , &quot;'&quot; , &quot;#password-strength-indicator&quot; , &quot;'&quot; , &quot;).removeClass();
      $(&quot; , &quot;'&quot; , &quot;#password-strength-indicator&quot; , &quot;'&quot; , &quot;).addClass(strength_label[0]);
      $(&quot; , &quot;'&quot; , &quot;#password-strength-indicator&quot; , &quot;'&quot; , &quot;).html(strength_label[1]);
      $(&quot; , &quot;'&quot; , &quot;.password-strength&quot; , &quot;'&quot; , &quot;).css(&quot; , &quot;'&quot; , &quot;display&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;block&quot; , &quot;'&quot; , &quot;);
    } else {
      $(&quot; , &quot;'&quot; , &quot;.password-strength&quot; , &quot;'&quot; , &quot;).css(&quot; , &quot;'&quot; , &quot;display&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;none&quot; , &quot;'&quot; , &quot;);
    }
  });
});


  
    $(&quot; , &quot;'&quot; , &quot;#name,#mobile,#password&quot; , &quot;'&quot; , &quot;).click(function(){
      if ($(&quot; , &quot;'&quot; , &quot;#country&quot; , &quot;'&quot; , &quot;).val() === &quot; , &quot;'&quot; , &quot;IN&quot; , &quot;'&quot; , &quot;){
        // Nav will not present inside an iframe
        if (window.self === window.top) {
          $(&quot; , &quot;'&quot; , &quot;body&quot; , &quot;'&quot; , &quot;).practoNav(&quot; , &quot;'&quot; , &quot;attemptTruecallerLogin&quot; , &quot;'&quot; , &quot;, {});
        }
      }
    });
  


      
        window.emptyMsg = &quot;field cannot be empty&quot;;
      
         
      &quot;) or . = concat(&quot;
        
        
      
      



  




  
   
      
        
          Join Practo
          
              
          
                Register Here
          
          Are you a doctor? 
          
        
      
   
   
    Full Name
    
        
         
  

    
    

    
        Mobile Number
        
            +91(IND)+65(SGP)+63(PHL)+60(MYS)+62(IDN)+55(BRA)+52(MEX)+54(ARG)+56(CHL)+84(VNM)+971(UAE)+965(KW)+255(TZA)+973(BH)+966(SA)+1(USA)
            
              
            
        
        
    

    
    
        Create Password
        
            
            
              Password Strength: Weak
            
             
  

        
    

    
      
      
      
      
      
   

   
      
        
         
            
            
         
         Receive relevant offers and promotional communication from Practo
         
             
                 
                 By signing up, I agree to terms
                 
             
         
        
      
      
      
        
        
        Send OTP
      
   
  
 




  


  #navCollapse,
 #bottomNavCollapse {
     background-color: #FFFFFF;
 }
 nav[role=&quot; , &quot;'&quot; , &quot;navigation&quot; , &quot;'&quot; , &quot;] {
     box-shadow: none;
     border: none;
 }
 .navbar-header {
     background-color: white;
 }
 .hrContainer {
     margin-top: 1%;
     margin-bottom: 1%;
     width: 410px;
 }
 .fbMsg {
  position: relative;
  top: -10px;
 }
 .join-practo {
   visibility: hidden;
 }
 #registerLink {
    color: #14bef0;
    font-weight: bold;
}
li > a#registerLink {
    border-bottom: 3px solid #14bef0;
  }
 #registerLink:hover {
  text-decoration: none;
 }
 .align-top {
  margin-top: 3px;
 }


window.registerCaptchaWidget = new RecaptchaWidget(
  &quot; , &quot;'&quot; , &quot;recaptcha-container-register&quot; , &quot;'&quot; , &quot;,
  function (recaptchaResponse) {
    document.getElementById(&quot; , &quot;'&quot; , &quot;register_email_form&quot; , &quot;'&quot; , &quot;).submit();
  }
);
  var registerEmailFn  =  function(event){
       var mobile = document.getElementById(&quot;mobile&quot;);
       var email = document.getElementById(&quot;email&quot;);
       var password =  document.getElementById(&quot;password&quot;);
       var name =  document.getElementById(&quot;name&quot;);
       var isPatientSignup = document.getElementById(&quot; , &quot;'&quot; , &quot;is-patient-signup&quot; , &quot;'&quot; , &quot;);
       if (mobile) {
        var mobileValidation = PRC.validator.Empty(mobile, null, &quot; , &quot;'&quot; , &quot;placeholder&quot; , &quot;'&quot; , &quot;);
       };
       if (email) {
        var emailValidation = PRC.validator.Empty(email, null, &quot; , &quot;'&quot; , &quot;placeholder&quot; , &quot;'&quot; , &quot;);
       }
       var passwordValidation = PRC.validator.Empty(password, null, &quot; , &quot;'&quot; , &quot;placeholder&quot; , &quot;'&quot; , &quot;);
       var nameErrorBlock = document.getElementById(&quot;nameErrorBlock&quot;);
       var mobileErrorBlock = document.getElementById(&quot;mobileErrorBlock&quot;);
       var emailErrorBlock = document.getElementById(&quot;emailErrorBlock&quot;);
       var passwordErrorBlock = document.getElementById(&quot;passwordErrorBlock&quot;);
       var hasError = false;

       if( typeof mobileValidation != &quot;undefined&quot; &amp;&amp; !mobileValidation){
           if(PRC.settings.gaTrackingCode  &amp;&amp; PRC.settings.gaTrackingCode.buttonClick){


              var pathName = window.location.pathname;
                if (pathName.indexOf(&quot; , &quot;'&quot; , &quot;patient&quot; , &quot;'&quot; , &quot;)>-1) {
                    PRC.settings.gaTrackingCode.buttonClick(&quot;PatRegFail_Email&quot;);
                  }else if(pathName.indexOf(&quot; , &quot;'&quot; , &quot;doctor&quot; , &quot;'&quot; , &quot;)>-1){
                      PRC.settings.gaTrackingCode.buttonClick(&quot;DocRegFail_Email&quot;);
                  }

          }

           mobileErrorBlock.innerHTML = mobile.errors[0];
           mobileErrorBlock.style.display=&quot;block&quot;;
           hasError = true;
           event.preventDefault();
       }
       else
          (typeof mobileErrorBlock != &quot;undefined&quot; &amp;&amp; mobileErrorBlock) ? mobileErrorBlock.style.cssText = &quot; &quot; : &quot;&quot;;

        if( typeof emailValidation != &quot;undefined&quot; &amp;&amp; !emailValidation){
           if(PRC.settings.gaTrackingCode  &amp;&amp; PRC.settings.gaTrackingCode.buttonClick){


              var pathName = window.location.pathname;
                if (pathName.indexOf(&quot; , &quot;'&quot; , &quot;patient&quot; , &quot;'&quot; , &quot;)>-1) {
                    PRC.settings.gaTrackingCode.buttonClick(&quot;PatRegFail_Email&quot;);
                  }else if(pathName.indexOf(&quot; , &quot;'&quot; , &quot;doctor&quot; , &quot;'&quot; , &quot;)>-1){
                      PRC.settings.gaTrackingCode.buttonClick(&quot;DocRegFail_Email&quot;);
                  }

          }

           emailErrorBlock.innerHTML = email.errors[0];
           emailErrorBlock.style.display=&quot;block&quot;;
           hasError = true;
           event.preventDefault();
       }
       else
          (typeof emailErrorBlock != &quot;undefined&quot; &amp;&amp; emailErrorBlock) ? emailErrorBlock.style.cssText = &quot; &quot; : &quot;&quot;;

        if(isPatientSignup) {
           var nameValidation = PRC.validator.Empty(name, null, &quot; , &quot;'&quot; , &quot;placeholder&quot; , &quot;'&quot; , &quot;);
           if(!nameValidation){
              nameErrorBlock.innerHTML = name.errors[0];
              nameErrorBlock.style.display=&quot;block&quot;;
              hasError = true;
              event.preventDefault();
           }
           else {
              name.value = name.value.trim();
              nameErrorBlock.innerHTML = &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;;
              nameErrorBlock.style.cssText = &quot; &quot;;
           }
           name.errors= [];
        }
       if(!passwordValidation){

              if(PRC.settings.gaTrackingCode  &amp;&amp; PRC.settings.gaTrackingCode.buttonClick){
                 var pathName = window.location.pathname;
                  if (pathName.indexOf(&quot; , &quot;'&quot; , &quot;patient&quot; , &quot;'&quot; , &quot;)>-1) {
                      PRC.settings.gaTrackingCode.buttonClick(&quot;PatRegFail_Email&quot;);
                    }else if(pathName.indexOf(&quot; , &quot;'&quot; , &quot;doctor&quot; , &quot;'&quot; , &quot;)>-1){
                        PRC.settings.gaTrackingCode.buttonClick(&quot;DocRegFail_Email&quot;);
                    }

              }

              passwordErrorBlock.innerHTML = password.errors[0];
              passwordErrorBlock.style.display=&quot;block&quot;;

              if(event.preventDefault)
                event.preventDefault();
              else{
                // event for enter button since event is nt associated for window object
                event.defaultPrevented =  true;
              }
              hasError = true;
       }
       else
        passwordErrorBlock.style.cssText = &quot; &quot;;

        if (!hasError) {
          var pathName = window.location.pathname;
          if (PRC.settings.gaTrackingCode  &amp;&amp; PRC.settings.gaTrackingCode.buttonClick) {
            if (pathName.indexOf(&quot; , &quot;'&quot; , &quot;patient&quot; , &quot;'&quot; , &quot;)>-1) {
              PRC.settings.gaTrackingCode.buttonClick(&quot;PatReg_Email&quot;);
            } else if(pathName.indexOf(&quot; , &quot;'&quot; , &quot;doctor&quot; , &quot;'&quot; , &quot;)>-1){
              PRC.settings.gaTrackingCode.buttonClick(&quot;DocReg_Email&quot;);
            }
          }
          // PEL -- Start
          if (typeof pel != &quot; , &quot;'&quot; , &quot;undefined&quot; , &quot;'&quot; , &quot; &amp;&amp; typeof pel.tracker != &quot; , &quot;'&quot; , &quot;undefined&quot; , &quot;'&quot; , &quot;) {
            var objectContext = { &quot; , &quot;'&quot; , &quot;Login Type&quot; , &quot;'&quot; , &quot;: &quot; , &quot;'&quot; , &quot;Phone&quot; , &quot;'&quot; , &quot; };
            pel.logEvent(&quot; , &quot;'&quot; , &quot;Login&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;Interacted&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;, objectContext);

            objectContext = {
              &quot; , &quot;'&quot; , &quot;Type&quot; , &quot;'&quot; , &quot;: &quot; , &quot;'&quot; , &quot;Notification&quot; , &quot;'&quot; , &quot;,
              &quot; , &quot;'&quot; , &quot;State&quot; , &quot;'&quot; , &quot;: $(&quot; , &quot;'&quot; , &quot;#subscribe_promotion&quot; , &quot;'&quot; , &quot;).prop(&quot; , &quot;'&quot; , &quot;checked&quot; , &quot;'&quot; , &quot;) === true ? &quot; , &quot;'&quot; , &quot;Yes&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;No&quot; , &quot;'&quot; , &quot;,
              &quot; , &quot;'&quot; , &quot;Is Logged In&quot; , &quot;'&quot; , &quot;: &quot; , &quot;'&quot; , &quot;No&quot; , &quot;'&quot; , &quot;
            };
            pel.logEvent(&quot; , &quot;'&quot; , &quot;User Setting&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;Updated&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;, objectContext);
          }
          // PEL -- End
          event.preventDefault();
          window.registerCaptchaWidget.render();
       }
       (mobile) ? mobile.errors= [] : &quot;&quot;;
       (email) ? email.errors= []: &quot;&quot;;
       password.errors= [];

       event.stopPropagation();
   };

 var checkMobileFn =  function(event){
         var mobile = document.getElementById(&quot;mobile&quot;);
         if (mobile) {
           var msg = &quot;Invalid Mobile Number&quot;;
           if(mobile.value.trim().length){
                var errorBlock =  document.getElementById(&quot;mobileErrorBlock&quot;);
              if(!PRC.validator.Empty(mobile, msg, &quot; , &quot;'&quot; , &quot;placeholder&quot; , &quot;'&quot; , &quot;) ){
                errorBlock.innerHTML = msg;
                errorBlock.style.display = &quot;block&quot;;

              }else{
                   errorBlock.innerHTML = &quot;&quot;;
                   errorBlock.style.display = &quot;none&quot;;

              }
              mobile.errors = [];
              event.preventDefault();
           }

           event.stopPropagation();
          }
   };

   var doctor  = function(event){
        if(PRC.settings.gaTrackingCode  &amp;&amp; PRC.settings.gaTrackingCode.linkClick){
            PRC.settings.gaTrackingCode.linkClick(&quot;DocReg_ForkFromPat&quot;, function(){
              window.location.replace(&quot; , &quot;'&quot; , &quot;/doctor_signup&quot; , &quot;'&quot; , &quot; + window.location.search);
            });
         }
         event.stopPropagation();
         event.preventDefault();
   };

   var notDoctor = function(event) {
             if(PRC.settings.gaTrackingCode  &amp;&amp; PRC.settings.gaTrackingCode.linkClick){
              PRC.settings.gaTrackingCode.linkClick(&quot;PatReg_ForkFromDoc&quot;, function(){
                var url = &quot; , &quot;'&quot; , &quot;/new_patient_signup&quot; , &quot;'&quot; , &quot; + window.location.search;
                window.location.replace(url.replace(&quot; , &quot;'&quot; , &quot;intent=software&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;intent=accounts&quot; , &quot;'&quot; , &quot;));
              });
           }
          event.stopPropagation();
          event.preventDefault();
   };

   var terms = function(event){
   };


   var obj = {
      init: function(eleIdArr){
            if(eleIdArr &amp;&amp; eleIdArr.length){
             var length = eleIdArr.length;
             for(var iloop=0; length--;iloop++){
                 var eleObj = eleIdArr[iloop];
                 if(document.getElementById(eleObj.eleId))
                     document.getElementById(eleObj.eleId).addEventListener(eleObj.event, this, eleObj.bubble?eleObj.bubble:false);
             }
           }

          window.addEventListener(&quot;keypress&quot;, function(event){
                if(event.keyCode == 13){
                  var registerEmailBtn =  document.getElementById(&quot;EmailRegister&quot;);
                  registerEmailBtn.onclick = registerEmailFn;
                  registerEmailBtn.onclick.apply(registerEmailBtn,[event]);
                }

          } , false);

      }
      ,handleEvent: function(event){
          switch(event.target.id){
              case &quot;EmailRegister&quot;:
                                    registerEmailFn(event);
                                    break;
              case &quot;EmailRegisterImage&quot;:
                                    registerEmailFn(event);
                                    break;

              case &quot;mobile&quot;        :
                                    checkMobileFn(event);
                                    break;

              case &quot;doctor&quot;        :
                                    doctor(event);
                                    break;

              case &quot;notDoctor&quot;     :
                                    notDoctor(event);
                                    break;
              case &quot;terms&quot;     :
                                    terms(event);
                                    break;

          };

      }
   };


   (function(eventObj){

      var eleArr = [
                       {&quot;eleId&quot;: &quot;EmailRegister&quot;, &quot;event&quot;: &quot;click&quot;},
                       {&quot;eleId&quot;: &quot;EmailRegisterImage&quot;, &quot;event&quot;: &quot;click&quot;},
                       {&quot;eleId&quot;: &quot;EmailRegister&quot;, &quot;event&quot;: &quot;click&quot;},
                       {&quot;eleId&quot;: &quot;EmailRegister&quot;, &quot;event&quot;: &quot;click&quot;},
                       {&quot;eleId&quot;: &quot;email&quot;, &quot;event&quot;: &quot;blur&quot;},
                       {&quot;eleId&quot;: &quot;doctor&quot;, &quot;event&quot;: &quot;click&quot;},
                       {&quot;eleId&quot;: &quot;notDoctor&quot;, &quot;event&quot;: &quot;click&quot;},
                       {&quot;eleId&quot;: &quot;mobile&quot;, &quot;event&quot;: &quot;blur&quot;},
                       {&quot;eleId&quot;: &quot;terms&quot;, &quot;event&quot;: &quot;click&quot;}
                     ];
      eventObj.init(eleArr);


    if(window.opener &amp;&amp; window.opener.PRC){
        window.opener.PRC.fbPopupFlag = true;
        window.opener.PRC.redirectURL = window.location.href.split(&quot; , &quot;'&quot; , &quot;#&quot; , &quot;'&quot; , &quot;)[0];
    }

   })(obj)
   if($(&quot;#mobileErrorBlock&quot;).text().trim()){
     $(&quot; , &quot;'&quot; , &quot;#mobile&quot; , &quot;'&quot; , &quot;).addClass(&quot; , &quot;'&quot; , &quot;borderRed&quot; , &quot;'&quot; , &quot;);
   }
   var joinPractoDoctor = document.getElementById(&quot; , &quot;'&quot; , &quot;joinPractoDoctor&quot; , &quot;'&quot; , &quot;);
   if(joinPractoDoctor ){
     var joinPractoDoctorHtml = joinPractoDoctor.innerHTML;
     if(joinPractoDoctorHtml.length > 21) {
       $(&quot; , &quot;'&quot; , &quot;.box-header&quot; , &quot;'&quot; , &quot;).css({
         &quot;font-size&quot;: &quot;14px&quot;
       });
     }
   }






$(function () {
  $(&quot; , &quot;'&quot; , &quot;#password&quot; , &quot;'&quot; , &quot;).on(&quot; , &quot;'&quot; , &quot;input&quot; , &quot;'&quot; , &quot;, function() {
    var password = $(this).val();
    if (password &amp;&amp; password.length &lt;= 72 &amp;&amp; password.length > 0 &amp;&amp; typeof zxcvbn !== &quot;undefined&quot;) {
      var user_dictionary = [
          &quot; , &quot;'&quot; , &quot;practo&quot; , &quot;'&quot; , &quot;,
          $(&quot; , &quot;'&quot; , &quot;#mobile&quot; , &quot;'&quot; , &quot;).val()
      ];
      Array.prototype.push.apply(user_dictionary, $(&quot; , &quot;'&quot; , &quot;#name&quot; , &quot;'&quot; , &quot;).val().split(&quot; , &quot;'&quot; , &quot; &quot; , &quot;'&quot; , &quot;));
      var result = zxcvbn(password, user_dictionary);
      var strength_label = [
        [&quot; , &quot;'&quot; , &quot;very-weak&quot; , &quot;'&quot; , &quot;, &quot;Very Weak&quot;],
        [&quot; , &quot;'&quot; , &quot;weak&quot; , &quot;'&quot; , &quot;, &quot;Weak&quot;],
        [&quot; , &quot;'&quot; , &quot;moderate&quot; , &quot;'&quot; , &quot;, &quot;Moderate&quot;],
        [&quot; , &quot;'&quot; , &quot;strong&quot; , &quot;'&quot; , &quot;, &quot;Strong&quot;],
        [&quot; , &quot;'&quot; , &quot;very-strong&quot; , &quot;'&quot; , &quot;, &quot;Very Strong&quot;]
      ][result.score];
      $(&quot; , &quot;'&quot; , &quot;#password-strength-indicator&quot; , &quot;'&quot; , &quot;).removeClass();
      $(&quot; , &quot;'&quot; , &quot;#password-strength-indicator&quot; , &quot;'&quot; , &quot;).addClass(strength_label[0]);
      $(&quot; , &quot;'&quot; , &quot;#password-strength-indicator&quot; , &quot;'&quot; , &quot;).html(strength_label[1]);
      $(&quot; , &quot;'&quot; , &quot;.password-strength&quot; , &quot;'&quot; , &quot;).css(&quot; , &quot;'&quot; , &quot;display&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;block&quot; , &quot;'&quot; , &quot;);
    } else {
      $(&quot; , &quot;'&quot; , &quot;.password-strength&quot; , &quot;'&quot; , &quot;).css(&quot; , &quot;'&quot; , &quot;display&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;none&quot; , &quot;'&quot; , &quot;);
    }
  });
});


  
    $(&quot; , &quot;'&quot; , &quot;#name,#mobile,#password&quot; , &quot;'&quot; , &quot;).click(function(){
      if ($(&quot; , &quot;'&quot; , &quot;#country&quot; , &quot;'&quot; , &quot;).val() === &quot; , &quot;'&quot; , &quot;IN&quot; , &quot;'&quot; , &quot;){
        // Nav will not present inside an iframe
        if (window.self === window.top) {
          $(&quot; , &quot;'&quot; , &quot;body&quot; , &quot;'&quot; , &quot;).practoNav(&quot; , &quot;'&quot; , &quot;attemptTruecallerLogin&quot; , &quot;'&quot; , &quot;, {});
        }
      }
    });
  


      
        window.emptyMsg = &quot;field cannot be empty&quot;;
      
         
      &quot;))]</value>
      <webElementGuid>eb74dbd9-43b7-4968-8ca9-ed623a9be8a7</webElementGuid>
   </webElementXpaths>
</WebElementEntity>
